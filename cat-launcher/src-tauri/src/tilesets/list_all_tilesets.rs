use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::File;
use tokio::fs::{read_dir, read_to_string};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::infra::utils::{OS, sort_assets};
use crate::tilesets::paths::{
  GetStockTilesetsDirError, get_stock_tilesets_dir,
  get_tilesets_resource_path,
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

  let third_party_tilesets =
    list_all_third_party_tilesets(game_variant, resource_dir).await?;
  tilesets.extend(third_party_tilesets);

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
  let tilesets_json_path = get_tilesets_resource_path(resource_dir);

  let content = match read_to_string(&tilesets_json_path).await {
    Ok(content) => content,
    Err(e) => {
      return Err(ListThirdPartyTilesetsError::ReadTilesetsJson(e));
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
        return Err(ListThirdPartyTilesetsError::ParseTilesetsJson(
          e,
        ));
      }
    }
  }

  Ok(tilesets)
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use strum::IntoEnumIterator;
  use tempfile::tempdir;

  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::filesystem::paths::get_or_create_asset_installation_dir;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::Asset;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_extract_stock_tileset_from_tileset_txt_valid()
  -> TestResult {
    let temp = tempdir()?;
    let tileset_file = temp.path().join("tileset.txt");
    let content = "# Tileset configuration\n# Comment\n\nNAME: ChestHoleTileset\nVIEW: ChestHole Tileset\n";
    tokio::fs::write(&tileset_file, content).await?;

    let ts =
      extract_stock_tileset_from_tileset_txt(&tileset_file).await?;
    assert_eq!(ts.id, "ChestHoleTileset");
    assert_eq!(ts.name, "ChestHole Tileset");

    Ok(())
  }

  #[tokio::test]
  async fn test_extract_stock_tileset_from_tileset_txt_missing_fields()
  -> TestResult {
    let temp = tempdir()?;

    // Missing VIEW
    let no_view = temp.path().join("no_view.txt");
    tokio::fs::write(&no_view, "NAME: only_name\n# no view here\n")
      .await?;
    assert!(matches!(
      extract_stock_tileset_from_tileset_txt(&no_view).await,
      Err(ExtractStockTilesetError::MissingField(f)) if f == "VIEW"
    ));

    // Missing NAME
    let no_name = temp.path().join("no_name.txt");
    tokio::fs::write(&no_name, "VIEW: only_view\n# no name here\n")
      .await?;
    assert!(matches!(
      extract_stock_tileset_from_tileset_txt(&no_name).await,
      Err(ExtractStockTilesetError::MissingField(f)) if f == "NAME"
    ));

    // Non-existent file
    let missing = temp.path().join("non_existent.txt");
    assert!(matches!(
      extract_stock_tileset_from_tileset_txt(&missing).await,
      Err(ExtractStockTilesetError::ReadTilesetTxt(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_stock_tilesets_dir() -> TestResult {
    let temp = tempdir()?;
    let stock_dir = temp.path();

    // Valid pack 1
    let pack1 = stock_dir.join("pack1");
    tokio::fs::create_dir_all(&pack1).await?;
    tokio::fs::write(
      pack1.join("tileset.txt"),
      "NAME: pack1\nVIEW: Tileset One\n",
    )
    .await?;

    // Valid pack 2
    let pack2 = stock_dir.join("pack2");
    tokio::fs::create_dir_all(&pack2).await?;
    tokio::fs::write(
      pack2.join("tileset.txt"),
      "NAME: pack2\nVIEW: Tileset Two\n",
    )
    .await?;

    // Invalid pack (missing VIEW)
    let pack_invalid = stock_dir.join("pack_invalid");
    tokio::fs::create_dir_all(&pack_invalid).await?;
    tokio::fs::write(
      pack_invalid.join("tileset.txt"),
      "NAME: invalid\n",
    )
    .await?;

    // Dir without tileset.txt
    let empty_dir = stock_dir.join("empty_dir");
    tokio::fs::create_dir_all(&empty_dir).await?;

    // Loose file (should be ignored)
    tokio::fs::write(stock_dir.join("readme.txt"), "ignore me")
      .await?;

    let stock_packs = list_all_stock_tilesets(stock_dir).await?;
    assert_eq!(stock_packs.len(), 2);
    let ids: Vec<&str> = stock_packs.iter().map(|s| s.id()).collect();
    assert!(ids.contains(&"pack1"));
    assert!(ids.contains(&"pack2"));

    // Non-existent dir returns empty vec
    let non_existent = temp.path().join("does_not_exist");
    let empty_result = list_all_stock_tilesets(&non_existent).await?;
    assert!(empty_result.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_tilesets_resource() -> TestResult
  {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;

    let json = serde_json::json!({
      "DarkDaysAhead": {
        "ts_a": {
          "id": "ts_a",
          "name": "Tileset A",
          "installation": {
            "download_url": "https://example.com/a.zip",
            "tileset": "tileset.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      },
      "BrightNights": {
        "ts_b": {
          "id": "ts_b",
          "name": "Tileset B",
          "installation": {
            "download_url": "https://example.com/b.zip",
            "tileset": "tileset.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });

    tokio::fs::write(
      content_dir.join("tilesets.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let dda = list_all_third_party_tilesets(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await?;
    assert_eq!(dda.len(), 1);
    assert_eq!(dda[0].id(), "ts_a");

    let bn = list_all_third_party_tilesets(
      &GameVariant::BrightNights,
      temp.path(),
    )
    .await?;
    assert_eq!(bn.len(), 1);
    assert_eq!(bn[0].id(), "ts_b");

    let tlg = list_all_third_party_tilesets(
      &GameVariant::TheLastGeneration,
      temp.path(),
    )
    .await?;
    assert!(tlg.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_tilesets_missing_file()
  -> TestResult {
    let temp = tempdir()?;
    let result = list_all_third_party_tilesets(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await;

    assert!(matches!(
      result,
      Err(ListThirdPartyTilesetsError::ReadTilesetsJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_tilesets_malformed_json()
  -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      "not valid json data",
    )
    .await?;

    let result = list_all_third_party_tilesets(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await;

    assert!(matches!(
      result,
      Err(ListThirdPartyTilesetsError::ParseTilesetsJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_tilesets_full_workflow() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // 1. Setup active release
    let version = "v0.H";
    active_release_repo
      .set_active_release(&variant, version)
      .await?;

    // 2. Setup stock tilesets on disk (out of alphabetical order)
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda-v0.H");
    let stock_gfx_dir = game_dir.join("gfx");

    let stock_pack_b = stock_gfx_dir.join("stock_ts_b");
    tokio::fs::create_dir_all(&stock_pack_b).await?;
    tokio::fs::write(
      stock_pack_b.join("tileset.txt"),
      "NAME: stock_ts_b\nVIEW: Stock Tileset B\n",
    )
    .await?;

    let stock_pack_a = stock_gfx_dir.join("stock_ts_a");
    tokio::fs::create_dir_all(&stock_pack_a).await?;
    tokio::fs::write(
      stock_pack_a.join("tileset.txt"),
      "NAME: stock_ts_a\nVIEW: Stock Tileset A\n",
    )
    .await?;

    // 3. Setup bundled tilesets.json (out of alphabetical order)
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      "DarkDaysAhead": {
        "tp_ts_b": {
          "id": "tp_ts_b",
          "name": "Third Party Tileset B",
          "installation": {
            "download_url": "https://example.com/tp_b.zip",
            "tileset": "tileset.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        },
        "tp_ts_a": {
          "id": "tp_ts_a",
          "name": "Third Party Tileset A",
          "installation": {
            "download_url": "https://example.com/tp_a.zip",
            "tileset": "tileset.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let result = list_all_tilesets(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
    )
    .await?;

    assert_eq!(result.len(), 4);
    // Third party should come first, sorted alphabetically by id
    assert!(result[0].is_third_party());
    assert_eq!(result[0].id(), "tp_ts_a");

    assert!(result[1].is_third_party());
    assert_eq!(result[1].id(), "tp_ts_b");

    // Stock tilesets follow, sorted alphabetically by id
    assert!(!result[2].is_third_party());
    assert_eq!(result[2].id(), "stock_ts_a");

    assert!(!result[3].is_third_party());
    assert_eq!(result[3].id(), "stock_ts_b");

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_tilesets_no_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // Bundled tilesets.json
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      "DarkDaysAhead": {
        "bundled_ts": {
          "id": "bundled_ts",
          "name": "Bundled Tileset",
          "installation": {
            "download_url": "https://example.com/b.zip",
            "tileset": "tileset.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let result = list_all_tilesets(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
    )
    .await?;

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id(), "bundled_ts");

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_tilesets_all_variants() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;

    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;

    let mut json_variants = serde_json::Map::new();
    for variant in GameVariant::iter() {
      let id = format!("ts_{}", variant);
      json_variants.insert(
        variant.to_string(),
        serde_json::json!({
          (id.clone()): {
            "id": id,
            "name": format!("Tileset {}", variant),
            "installation": {
              "download_url": format!("https://example.com/{}.zip", variant),
              "tileset": "tileset.txt"
            },
            "activity": {
              "activity_type": "github",
              "github": null
            }
          }
        }),
      );
    }
    let json = serde_json::Value::Object(json_variants);

    tokio::fs::write(
      content_dir.join("tilesets.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    for variant in GameVariant::iter() {
      let expected_id = format!("ts_{}", variant);
      let result = list_all_tilesets(
        &variant,
        temp_data.path(),
        temp_res.path(),
        &os,
        &active_release_repo,
      )
      .await?;

      assert_eq!(result.len(), 1);
      assert_eq!(result[0].id(), expected_id);
    }

    Ok(())
  }
}
