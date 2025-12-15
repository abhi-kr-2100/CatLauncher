use std::io;
use std::path::Path;

use tokio::fs::{read_dir, read_to_string};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::infra::utils::OS;
use crate::mods::paths::{get_stock_mods_dir, GetStockModsDirError};
use crate::mods::types::{Mod, StockMod, ThirdPartyMod};
use crate::mods::utils::{
  get_third_party_mods_json, GetThirdPartyModsJsonError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListAllModsError {
  #[error("failed to get stock mods dir: {0}")]
  GetStockModsDir(#[from] GetStockModsDirError),

  #[error("failed to read stock mod dir: {0}")]
  ExtractStockMod(#[from] ListAllStockModsError),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  #[error("failed to list third-party mods: {0}")]
  ListThirdPartyMods(#[from] ListThirdPartyModsError),
}

pub async fn list_all_mods(
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
) -> Result<Vec<Mod>, ListAllModsError> {
  let mut mods = Vec::new();

  // Add third-party mods
  let third_party_mods =
    list_all_third_party_mods(game_variant, resource_dir).await?;
  mods.extend(third_party_mods);

  // Add stock mods
  let release_version = active_release_repository
    .get_active_release(game_variant)
    .await?;

  if let Some(release_version) = release_version {
    let stock_mods_dir = get_stock_mods_dir(
      game_variant,
      &release_version,
      data_dir,
      os,
    )
    .await?;
    let stock_mods = list_all_stock_mods(&stock_mods_dir).await?;
    mods.extend(stock_mods);
  }

  Ok(mods)
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractStockModError {
  #[error("failed to parse modinfo.json: {0}")]
  ParseModInfo(#[from] serde_json::Error),

  #[error("no MOD_INFO entry found")]
  NoModInfo,

  #[error("modinfo missing required field: {0}")]
  MissingField(String),
}

fn extract_stock_mod_from_modinfo(
  modinfo_content: &str,
) -> Result<StockMod, ExtractStockModError> {
  let entries: Vec<serde_json::Value> =
    serde_json::from_str(modinfo_content)?;

  // Find the MOD_INFO entry
  let mod_info = entries
    .iter()
    .find(|entry| {
      entry.get("type").and_then(|t| t.as_str()) == Some("MOD_INFO")
    })
    .ok_or(ExtractStockModError::NoModInfo)?;

  let id = mod_info
    .get("id")
    .and_then(|i| i.as_str())
    .ok_or(ExtractStockModError::MissingField("id".to_string()))?
    .to_string();

  let name = mod_info
    .get("name")
    .and_then(|n| n.as_str())
    .ok_or(ExtractStockModError::MissingField("name".to_string()))?
    .to_string();

  let description = mod_info
    .get("description")
    .and_then(|d| d.as_str())
    .unwrap_or("")
    .to_string();

  let category = mod_info
    .get("category")
    .and_then(|c| c.as_str())
    .unwrap_or("")
    .to_string();

  Ok(StockMod {
    id,
    name,
    description,
    category,
  })
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllStockModsError {
  #[error("failed to read stock mods directory: {0}")]
  ReadDir(#[from] io::Error),
}

async fn list_all_stock_mods(
  stock_mods_dir: &Path,
) -> Result<Vec<Mod>, ListAllStockModsError> {
  let mut mods = Vec::new();
  let mut dir_entries = read_dir(stock_mods_dir).await?;

  while let Some(entry) = dir_entries.next_entry().await? {
    let path = entry.path();

    if !entry.file_type().await?.is_dir() {
      continue;
    }

    let modinfo_path = path.join("modinfo.json");

    match read_to_string(&modinfo_path).await {
      Ok(content) => match extract_stock_mod_from_modinfo(&content) {
        Ok(stock_mod) => {
          mods.push(Mod::Stock(stock_mod));
        }
        Err(_) => {
          // Failed to parse modinfo.json, skip this entry
          continue;
        }
      },
      Err(_) => {
        // This directory doesn't have a modinfo.json file, skip it
        continue;
      }
    }
  }

  Ok(mods)
}

#[derive(thiserror::Error, Debug)]
pub enum ListThirdPartyModsError {
  #[error("failed to get third party mods json: {0}")]
  GetThirdPartyModsJson(#[from] GetThirdPartyModsJsonError),

  #[error("failed to parse mod: {0}")]
  ParseMod(#[from] serde_json::Error),
}

async fn list_all_third_party_mods(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<Mod>, ListThirdPartyModsError> {
  let mods_data = get_third_party_mods_json(resource_dir).await?;

  let variant_mods = match mods_data.get(game_variant) {
    Some(mods) => mods,
    None => return Ok(Vec::new()),
  };

  let mut mods = Vec::new();
  for mod_data in variant_mods.values() {
    let third_party_mod =
      serde_json::from_value::<ThirdPartyMod>(mod_data.clone())?;
    mods.push(Mod::ThirdParty(third_party_mod))
  }

  Ok(mods)
}
