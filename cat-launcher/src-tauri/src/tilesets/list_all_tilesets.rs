use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::File;
use tokio::fs::{read_dir, read_to_string};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::infra::utils::{sort_assets, OS};
use crate::tilesets::paths::{
  get_stock_tilesets_dir, get_tilesets_resource_path,
  GetStockTilesetsDirError,
};
use crate::tilesets::types::{
  StockTileset, ThirdPartyTileset, Tileset,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListAllTilesetsError {
  #[error("failed to get stock tilesets dir: {0}")]
  GetStockTilesetsDir(#[from] GetStockTilesetsDirError),

  #[error("failed to read stock tileset dir: {0}")]
  ExtractStockTileset(#[from] ListAllStockTilesetsError),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  #[error("failed to list third-party tilesets: {0}")]
  ListThirdPartyTilesets(#[from] ListThirdPartyTilesetsError),
}

pub async fn list_all_tilesets(
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
) -> Result<Vec<Tileset>, ListAllTilesetsError> {
  let mut tilesets = Vec::new();

  // Add third-party tilesets
  let third_party_tilesets =
    list_all_third_party_tilesets(game_variant, resource_dir).await?;
  tilesets.extend(third_party_tilesets);

  // Add stock tilesets
  let release_version = active_release_repository
    .get_active_release(game_variant)
    .await?;

  if let Some(release_version) = release_version {
    let stock_tilesets_dir = get_stock_tilesets_dir(
      game_variant,
      &release_version,
      data_dir,
      os,
    )
    .await?;
    let stock_tilesets =
      list_all_stock_tilesets(&stock_tilesets_dir).await?;
    tilesets.extend(stock_tilesets);
  }

  sort_assets(&mut tilesets);

  Ok(tilesets)
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractStockTilesetError {
  #[error("failed to read tileset.txt: {0}")]
  ReadTilesetTxt(#[from] io::Error),
  #[error("missing field in tileset.txt: {0}")]
  MissingField(String),
}

async fn extract_stock_tileset_from_tileset_txt(
  tileset_txt_path: &Path,
) -> Result<StockTileset, ExtractStockTilesetError> {
  let file = File::open(tileset_txt_path).await?;
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
    ExtractStockTilesetError::MissingField("NAME".to_string())
  })?;
  let name = view.ok_or_else(|| {
    ExtractStockTilesetError::MissingField("VIEW".to_string())
  })?;

  Ok(StockTileset { id, name })
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllStockTilesetsError {
  #[error("failed to read stock tilesets directory: {0}")]
  ReadDir(#[from] io::Error),
}

async fn list_all_stock_tilesets(
  stock_tilesets_dir: &Path,
) -> Result<Vec<Tileset>, ListAllStockTilesetsError> {
  let mut tilesets = Vec::new();
  if !stock_tilesets_dir.exists() {
    return Ok(tilesets);
  }
  let mut dir_entries = read_dir(stock_tilesets_dir).await?;

  while let Some(entry) = dir_entries.next_entry().await? {
    let path = entry.path();

    if !entry.file_type().await?.is_dir() {
      continue;
    }

    let tileset_txt_path = path.join("tileset.txt");

    if tileset_txt_path.exists() {
      match extract_stock_tileset_from_tileset_txt(&tileset_txt_path)
        .await
      {
        Ok(stock_tileset) => {
          tilesets.push(Tileset::Stock(stock_tileset));
        }
        Err(_) => {
          // Failed to parse, skip
          continue;
        }
      }
    }
  }

  Ok(tilesets)
}

#[derive(thiserror::Error, Debug)]
pub enum ListThirdPartyTilesetsError {
  #[error("failed to read tilesets.json: {0}")]
  ReadTilesetsJson(#[from] io::Error),

  #[error("failed to parse tilesets.json: {0}")]
  ParseTilesetsJson(#[from] serde_json::Error),
}

async fn list_all_third_party_tilesets(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<Tileset>, ListThirdPartyTilesetsError> {
  // Construct the path to tilesets.json
  let tilesets_json_path = get_tilesets_resource_path(resource_dir);

  // Try to read the tilesets.json file
  let content = match read_to_string(&tilesets_json_path).await {
    Ok(content) => content,
    Err(e) => {
      return Err(ListThirdPartyTilesetsError::ReadTilesetsJson(e))
    }
  };

  let tilesets_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_tilesets = match tilesets_data.get(game_variant) {
    Some(tilesets) => tilesets,
    None => return Ok(Vec::new()),
  };

  let mut tilesets = Vec::new();
  for tileset_data in variant_tilesets.values() {
    let third_party_tileset = serde_json::from_value::<
      ThirdPartyTileset,
    >(tileset_data.clone());
    match third_party_tileset {
      Ok(third_party_tileset) => {
        tilesets.push(Tileset::ThirdParty(third_party_tileset))
      }
      Err(e) => {
        return Err(ListThirdPartyTilesetsError::ParseTilesetsJson(e))
      }
    }
  }

  Ok(tilesets)
}
