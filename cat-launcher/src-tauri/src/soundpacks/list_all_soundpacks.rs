use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::File;
use tokio::fs::{read_dir, read_to_string};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::infra::utils::OS;
use crate::soundpacks::paths::{
  get_soundpacks_resource_path, get_stock_soundpacks_dir,
  GetStockSoundpacksDirError,
};
use crate::soundpacks::types::{
  Soundpack, StockSoundpack, ThirdPartySoundpack,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListAllSoundpacksError {
  #[error("failed to get stock soundpacks dir: {0}")]
  GetStockSoundpacksDir(#[from] GetStockSoundpacksDirError),

  #[error("failed to read stock soundpack dir: {0}")]
  ExtractStockSoundpack(#[from] ListAllStockSoundpacksError),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  #[error("failed to list third-party soundpacks: {0}")]
  ListThirdPartySoundpacks(#[from] ListThirdPartySoundpacksError),
}

pub async fn list_all_soundpacks(
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
) -> Result<Vec<Soundpack>, ListAllSoundpacksError> {
  let mut soundpacks = Vec::new();

  // Add third-party soundpacks
  let third_party_soundpacks =
    list_all_third_party_soundpacks(game_variant, resource_dir)
      .await?;
  soundpacks.extend(third_party_soundpacks);

  // Add stock soundpacks
  let release_version = active_release_repository
    .get_active_release(game_variant)
    .await?;

  if let Some(release_version) = release_version {
    let stock_soundpacks_dir = get_stock_soundpacks_dir(
      game_variant,
      &release_version,
      data_dir,
      os,
    )
    .await?;
    let stock_soundpacks =
      list_all_stock_soundpacks(&stock_soundpacks_dir).await?;
    soundpacks.extend(stock_soundpacks);
  }

  Ok(soundpacks)
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractStockSoundpackError {
  #[error("failed to read soundpack.txt: {0}")]
  ReadSoundpackTxt(#[from] io::Error),
  #[error("missing field in soundpack.txt: {0}")]
  MissingField(String),
}

async fn extract_stock_soundpack_from_soundpack_txt(
  soundpack_txt_path: &Path,
) -> Result<StockSoundpack, ExtractStockSoundpackError> {
  let file = File::open(soundpack_txt_path).await?;
  let reader = BufReader::new(file);
  let mut lines = reader.lines();

  let mut name = None;
  let mut view = None;

  while let Some(line) = lines.next_line().await? {
    if line.starts_with('#') || line.trim().is_empty() {
      continue;
    }

    if let Some(captures) = line.split_once(':') {
      let key = captures.0.trim();
      let value = captures.1.trim().to_string();

      if key == "NAME" {
        name = Some(value);
      } else if key == "VIEW" {
        view = Some(value);
      }
    }
  }

  let id = name.ok_or_else(|| {
    ExtractStockSoundpackError::MissingField("NAME".to_string())
  })?;
  let name = view.ok_or_else(|| {
    ExtractStockSoundpackError::MissingField("VIEW".to_string())
  })?;

  Ok(StockSoundpack { id, name })
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllStockSoundpacksError {
  #[error("failed to read stock soundpacks directory: {0}")]
  ReadDir(#[from] io::Error),
}

async fn list_all_stock_soundpacks(
  stock_soundpacks_dir: &Path,
) -> Result<Vec<Soundpack>, ListAllStockSoundpacksError> {
  let mut soundpacks = Vec::new();
  if !stock_soundpacks_dir.exists() {
    return Ok(soundpacks);
  }
  let mut dir_entries = read_dir(stock_soundpacks_dir).await?;

  while let Some(entry) = dir_entries.next_entry().await? {
    let path = entry.path();

    if !entry.file_type().await?.is_dir() {
      continue;
    }

    let soundpack_txt_path = path.join("soundpack.txt");

    if soundpack_txt_path.exists() {
      match extract_stock_soundpack_from_soundpack_txt(
        &soundpack_txt_path,
      )
      .await
      {
        Ok(stock_soundpack) => {
          soundpacks.push(Soundpack::Stock(stock_soundpack));
        }
        Err(_) => {
          // Failed to parse, skip
          continue;
        }
      }
    }
  }

  Ok(soundpacks)
}

#[derive(thiserror::Error, Debug)]
pub enum ListThirdPartySoundpacksError {
  #[error("failed to read soundpacks.json: {0}")]
  ReadSoundpacksJson(#[from] io::Error),

  #[error("failed to parse soundpacks.json: {0}")]
  ParseSoundpacksJson(#[from] serde_json::Error),
}

async fn list_all_third_party_soundpacks(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<Soundpack>, ListThirdPartySoundpacksError> {
  // Construct the path to soundpacks.json
  let soundpacks_json_path =
    get_soundpacks_resource_path(resource_dir);

  // Try to read the soundpacks.json file
  let content = match read_to_string(&soundpacks_json_path).await {
    Ok(content) => content,
    Err(e) => {
      return Err(ListThirdPartySoundpacksError::ReadSoundpacksJson(
        e,
      ))
    }
  };

  let soundpacks_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_soundpacks = match soundpacks_data.get(game_variant) {
    Some(soundpacks) => soundpacks,
    None => return Ok(Vec::new()),
  };

  let mut soundpacks = Vec::new();
  for soundpack_data in variant_soundpacks.values() {
    let third_party_soundpack = serde_json::from_value::<
      ThirdPartySoundpack,
    >(soundpack_data.clone());
    match third_party_soundpack {
      Ok(third_party_soundpack) => {
        soundpacks.push(Soundpack::ThirdParty(third_party_soundpack))
      }
      Err(e) => {
        return Err(
          ListThirdPartySoundpacksError::ParseSoundpacksJson(e),
        )
      }
    }
  }

  Ok(soundpacks)
}
