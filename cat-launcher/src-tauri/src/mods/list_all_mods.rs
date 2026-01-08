use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::path::Path;

use tokio::fs::{read_dir, read_to_string};

use crate::active_release::repository::ActiveReleaseRepository;
use crate::infra::utils::{sort_assets, OS};
use crate::mods::lib::{
  get_mods_resource_path, get_stock_mods_dir, GetStockModsDirError,
};
use crate::mods::online::types::OnlineModRepository;
use crate::mods::repository::mods_repository::{
  ListCachedThirdPartyModsError, ModsRepository,
  SaveThirdPartyModsError,
};
use crate::mods::types::{
  FetchOnlineModsError, Mod, ModsUpdatePayload, ModsUpdateStatus,
  StockMod, ThirdPartyMod,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetAllStockModsError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to get stock mods dir: {0}")]
  GetStockModsDir(#[from] GetStockModsDirError),

  #[error("failed to read stock mod dir: {0}")]
  ListStockMods(#[from] ListAllStockModsError),
}

pub async fn get_all_stock_mods(
  game_variant: &GameVariant,
  data_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
) -> Result<Vec<StockMod>, GetAllStockModsError> {
  let release_version = active_release_repository
    .get_active_release(game_variant)
    .await
    .map_err(|e| {
      GetAllStockModsError::GetActiveRelease(Box::new(e))
    })?;

  if let Some(release_version) = release_version {
    let stock_mods_dir = get_stock_mods_dir(
      game_variant,
      &release_version,
      data_dir,
      os,
    )
    .await?;

    let stock_mods = list_all_stock_mods(&stock_mods_dir).await?;
    Ok(stock_mods)
  } else {
    Ok(Vec::new())
  }
}

#[derive(thiserror::Error, Debug)]
pub enum ListAllModsError<E: std::error::Error> {
  #[error("failed to get stock mods dir: {0}")]
  GetStockModsDir(#[from] GetStockModsDirError),

  #[error("failed to read stock mod dir: {0}")]
  ExtractStockMod(#[from] ListAllStockModsError),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to list third-party mods: {0}")]
  ListThirdPartyMods(#[from] ListThirdPartyModsError),

  #[error("failed to send update: {0}")]
  Send(E),

  #[error("failed to fetch online mods: {0}")]
  OnlineFetch(#[from] FetchOnlineModsError),

  #[error("failed to save mods to repository: {0}")]
  Repository(#[from] SaveThirdPartyModsError),

  #[error("failed to list cached third-party mods: {0}")]
  ListCachedMods(#[from] ListCachedThirdPartyModsError),
}

#[allow(clippy::too_many_arguments)]
pub async fn list_all_mods<F, E>(
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
  mods_repository: &impl ModsRepository,
  online_mod_repositories: &[Box<dyn OnlineModRepository>],
  client: &reqwest::Client,
  on_update: F,
) -> Result<(), ListAllModsError<E>>
where
  E: std::error::Error,
  F: Fn(ModsUpdatePayload) -> Result<(), E>,
{
  let mut all_mods = Vec::new();
  let mut mod_ids = HashSet::new();

  // 1. First, get cached mods from database and emit them.
  let cached_mods =
    list_cached_third_party_mods(game_variant, mods_repository)
      .await
      .map_err(ListAllModsError::ListCachedMods)?;

  for cached_mod in cached_mods {
    mod_ids.insert(cached_mod.id.clone());
    all_mods.push(Mod::ThirdParty(cached_mod));
  }

  sort_assets(&mut all_mods);

  on_update(ModsUpdatePayload {
    variant: *game_variant,
    mods: all_mods.clone(),
    status: ModsUpdateStatus::Fetching,
  })
  .map_err(ListAllModsError::Send)?;

  // 2. Fetch online mods from all registries and emit them.
  for repo in online_mod_repositories {
    let online_mods =
      repo.get_mods_for_variant(game_variant, client).await?;

    let mut new_mods_added = false;
    for tp_mod in online_mods {
      if mod_ids.insert(tp_mod.id.clone()) {
        all_mods.push(Mod::ThirdParty(tp_mod));
        new_mods_added = true;
      }
    }

    if new_mods_added {
      sort_assets(&mut all_mods);

      on_update(ModsUpdatePayload {
        variant: *game_variant,
        mods: all_mods.clone(),
        status: ModsUpdateStatus::Fetching,
      })
      .map_err(ListAllModsError::Send)?;
    }
  }

  // 3. Add bundled third-party mods and stock mods, then emit final success.
  let local_third_party_mods =
    list_all_third_party_mods(game_variant, resource_dir)
      .await
      .map_err(ListAllModsError::ListThirdPartyMods)?;

  for tp in local_third_party_mods {
    if mod_ids.insert(tp.id.clone()) {
      all_mods.push(Mod::ThirdParty(tp));
    }
  }

  let stock_mods = get_all_stock_mods(
    game_variant,
    data_dir,
    os,
    active_release_repository,
  )
  .await
  .map_err(|e| ListAllModsError::GetActiveRelease(Box::new(e)))?;

  for sp in stock_mods {
    all_mods.push(Mod::Stock(sp));
  }

  sort_assets(&mut all_mods);

  // Save all discovered third-party mods to DB
  mods_repository
    .save_third_party_mods(
      game_variant,
      all_mods
        .iter()
        .filter_map(|m| {
          if let Mod::ThirdParty(tp) = m {
            Some(tp.clone())
          } else {
            None
          }
        })
        .collect(),
    )
    .await?;

  // Emit final list
  on_update(ModsUpdatePayload {
    variant: *game_variant,
    mods: all_mods,
    status: ModsUpdateStatus::Success,
  })
  .map_err(ListAllModsError::Send)?;

  Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractStockModError {
  #[error("failed to parse modinfo.json: {0}")]
  ParseModInfo(#[from] serde_json::Error),

  #[error("no MOD_INFO entry found")]
  NoModInfo,

  #[error("modinfo missing required field: {0}")]
  MissingField(&'static str),
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
    .ok_or(ExtractStockModError::MissingField("id"))?
    .to_string();

  let name = mod_info
    .get("name")
    .and_then(|n| n.as_str())
    .ok_or(ExtractStockModError::MissingField("name"))?
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
) -> Result<Vec<StockMod>, ListAllStockModsError> {
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
          mods.push(stock_mod);
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
  #[error("failed to read mods.json: {0}")]
  ReadModsJson(#[from] io::Error),

  #[error("failed to parse mods.json: {0}")]
  ParseModsJson(#[from] serde_json::Error),
}

pub async fn list_all_third_party_mods(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<ThirdPartyMod>, ListThirdPartyModsError> {
  // Construct the path to mods.json
  let mods_json_path = get_mods_resource_path(resource_dir);

  // Try to read the mods.json file
  let content = match read_to_string(&mods_json_path).await {
    Ok(content) => content,
    Err(e) => return Err(ListThirdPartyModsError::ReadModsJson(e)),
  };

  let mods_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_mods = match mods_data.get(game_variant) {
    Some(mods) => mods,
    None => return Ok(Vec::new()),
  };

  let mut mods = Vec::new();
  for mod_data in variant_mods.values() {
    let third_party_mod =
      serde_json::from_value::<ThirdPartyMod>(mod_data.clone());
    match third_party_mod {
      Ok(third_party_mod) => mods.push(third_party_mod),
      Err(e) => {
        return Err(ListThirdPartyModsError::ParseModsJson(e))
      }
    }
  }

  Ok(mods)
}

pub async fn list_cached_third_party_mods(
  game_variant: &GameVariant,
  mods_repository: &impl ModsRepository,
) -> Result<Vec<ThirdPartyMod>, ListCachedThirdPartyModsError> {
  mods_repository.get_third_party_mods(game_variant).await
}
