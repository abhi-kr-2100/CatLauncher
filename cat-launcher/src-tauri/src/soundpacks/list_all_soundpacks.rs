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
use crate::soundpacks::paths::{
  GetStockSoundpacksDirError, get_soundpacks_resource_path,
  get_stock_soundpacks_dir,
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

  let third_party_soundpacks =
    list_all_third_party_soundpacks(game_variant, resource_dir)
      .await?;
  soundpacks.extend(third_party_soundpacks);

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

  sort_assets(&mut soundpacks);

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
  let soundpacks_json_path =
    get_soundpacks_resource_path(resource_dir);

  let content = match read_to_string(&soundpacks_json_path).await {
    Ok(content) => content,
    Err(e) => {
      return Err(ListThirdPartySoundpacksError::ReadSoundpacksJson(
        e,
      ));
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
        );
      }
    }
  }

  Ok(soundpacks)
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
  use tempfile::tempdir;

  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::filesystem::paths::get_or_create_asset_installation_dir;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::Asset;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_extract_stock_soundpack_from_soundpack_txt_valid()
  -> TestResult {
    let temp = tempdir()?;
    let soundpack_file = temp.path().join("soundpack.txt");
    let content = "# Soundpack configuration\n# Comment\n\nNAME: ChestHoleSoundPack\nVIEW: ChestHole SoundPack\n";
    tokio::fs::write(&soundpack_file, content).await?;

    let sp =
      extract_stock_soundpack_from_soundpack_txt(&soundpack_file)
        .await?;
    assert_eq!(sp.id, "ChestHoleSoundPack");
    assert_eq!(sp.name, "ChestHole SoundPack");

    Ok(())
  }

  #[tokio::test]
  async fn test_extract_stock_soundpack_from_soundpack_txt_missing_fields()
  -> TestResult {
    let temp = tempdir()?;

    // Missing VIEW
    let no_view = temp.path().join("no_view.txt");
    tokio::fs::write(&no_view, "NAME: only_name\n# no view here\n")
      .await?;
    assert!(matches!(
      extract_stock_soundpack_from_soundpack_txt(&no_view).await,
      Err(ExtractStockSoundpackError::MissingField(f)) if f == "VIEW"
    ));

    // Missing NAME
    let no_name = temp.path().join("no_name.txt");
    tokio::fs::write(&no_name, "VIEW: only_view\n# no name here\n")
      .await?;
    assert!(matches!(
      extract_stock_soundpack_from_soundpack_txt(&no_name).await,
      Err(ExtractStockSoundpackError::MissingField(f)) if f == "NAME"
    ));

    // Non-existent file
    let missing = temp.path().join("non_existent.txt");
    assert!(matches!(
      extract_stock_soundpack_from_soundpack_txt(&missing).await,
      Err(ExtractStockSoundpackError::ReadSoundpackTxt(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_stock_soundpacks_dir() -> TestResult {
    let temp = tempdir()?;
    let stock_dir = temp.path();

    // Valid pack 1
    let pack1 = stock_dir.join("pack1");
    tokio::fs::create_dir_all(&pack1).await?;
    tokio::fs::write(
      pack1.join("soundpack.txt"),
      "NAME: pack1\nVIEW: Soundpack One\n",
    )
    .await?;

    // Valid pack 2
    let pack2 = stock_dir.join("pack2");
    tokio::fs::create_dir_all(&pack2).await?;
    tokio::fs::write(
      pack2.join("soundpack.txt"),
      "NAME: pack2\nVIEW: Soundpack Two\n",
    )
    .await?;

    // Invalid pack (missing VIEW)
    let pack_invalid = stock_dir.join("pack_invalid");
    tokio::fs::create_dir_all(&pack_invalid).await?;
    tokio::fs::write(
      pack_invalid.join("soundpack.txt"),
      "NAME: invalid\n",
    )
    .await?;

    // Dir without soundpack.txt
    let empty_dir = stock_dir.join("empty_dir");
    tokio::fs::create_dir_all(&empty_dir).await?;

    // Loose file (should be ignored)
    tokio::fs::write(stock_dir.join("readme.txt"), "ignore me")
      .await?;

    let stock_packs = list_all_stock_soundpacks(stock_dir).await?;
    assert_eq!(stock_packs.len(), 2);
    let ids: Vec<&str> = stock_packs.iter().map(|s| s.id()).collect();
    assert!(ids.contains(&"pack1"));
    assert!(ids.contains(&"pack2"));

    // Non-existent dir returns empty vec
    let non_existent = temp.path().join("does_not_exist");
    let empty_result =
      list_all_stock_soundpacks(&non_existent).await?;
    assert!(empty_result.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_soundpacks_resource()
  -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;

    let json = serde_json::json!({
      "DarkDaysAhead": {
        "sp_a": {
          "id": "sp_a",
          "name": "Soundpack A",
          "installation": {
            "download_url": "https://example.com/a.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      },
      "BrightNights": {
        "sp_b": {
          "id": "sp_b",
          "name": "Soundpack B",
          "installation": {
            "download_url": "https://example.com/b.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });

    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let dda = list_all_third_party_soundpacks(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await?;
    assert_eq!(dda.len(), 1);
    assert_eq!(dda[0].id(), "sp_a");

    let bn = list_all_third_party_soundpacks(
      &GameVariant::BrightNights,
      temp.path(),
    )
    .await?;
    assert_eq!(bn.len(), 1);
    assert_eq!(bn[0].id(), "sp_b");

    let tlg = list_all_third_party_soundpacks(
      &GameVariant::TheLastGeneration,
      temp.path(),
    )
    .await?;
    assert!(tlg.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_soundpacks_missing_file()
  -> TestResult {
    let temp = tempdir()?;
    let result = list_all_third_party_soundpacks(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await;

    assert!(matches!(
      result,
      Err(ListThirdPartySoundpacksError::ReadSoundpacksJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_soundpacks_malformed_json()
  -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      "not valid json data",
    )
    .await?;

    let result = list_all_third_party_soundpacks(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await;

    assert!(matches!(
      result,
      Err(ListThirdPartySoundpacksError::ParseSoundpacksJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_soundpacks_full_workflow() -> TestResult {
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

    // 2. Setup stock soundpack on disk
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda-v0.H");
    let stock_sound_dir = game_dir.join("data").join("sound");
    let stock_pack_dir = stock_sound_dir.join("stock_sp");
    tokio::fs::create_dir_all(&stock_pack_dir).await?;
    tokio::fs::write(
      stock_pack_dir.join("soundpack.txt"),
      "NAME: stock_sp\nVIEW: Stock Soundpack\n",
    )
    .await?;

    // 3. Setup bundled soundpacks.json
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      "DarkDaysAhead": {
        "tp_sp": {
          "id": "tp_sp",
          "name": "Third Party Soundpack",
          "installation": {
            "download_url": "https://example.com/tp.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let result = list_all_soundpacks(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
    )
    .await?;

    assert_eq!(result.len(), 2);
    // Third party should come first due to sort_assets
    assert!(result[0].is_third_party());
    assert_eq!(result[0].id(), "tp_sp");

    assert!(!result[1].is_third_party());
    assert_eq!(result[1].id(), "stock_sp");

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_soundpacks_no_active_release() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // Bundled soundpacks.json
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      "DarkDaysAhead": {
        "bundled_sp": {
          "id": "bundled_sp",
          "name": "Bundled Soundpack",
          "installation": {
            "download_url": "https://example.com/b.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let result = list_all_soundpacks(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
    )
    .await?;

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id(), "bundled_sp");
    assert!(result[0].is_third_party());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_soundpacks_sorting() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // Setup active release
    let version = "v0.H";
    active_release_repo
      .set_active_release(&variant, version)
      .await?;

    // Stock packs: "stock_z" and "stock_a"
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda-v0.H");
    let sound_dir = game_dir.join("data").join("sound");

    let sz = sound_dir.join("stock_z");
    tokio::fs::create_dir_all(&sz).await?;
    tokio::fs::write(
      sz.join("soundpack.txt"),
      "NAME: stock_z\nVIEW: Z\n",
    )
    .await?;

    let sa = sound_dir.join("stock_a");
    tokio::fs::create_dir_all(&sa).await?;
    tokio::fs::write(
      sa.join("soundpack.txt"),
      "NAME: stock_a\nVIEW: A\n",
    )
    .await?;

    // Third party packs: "tp_z" and "tp_a"
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      "DarkDaysAhead": {
        "tp_z": {
          "id": "tp_z",
          "name": "TP Z",
          "installation": {
            "download_url": "https://example.com/z.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        },
        "tp_a": {
          "id": "tp_a",
          "name": "TP A",
          "installation": {
            "download_url": "https://example.com/a.zip",
            "soundpack": "soundpack.txt"
          },
          "activity": {
            "activity_type": "github",
            "github": null
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      serde_json::to_string(&json)?,
    )
    .await?;

    let result = list_all_soundpacks(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
    )
    .await?;

    let ids: Vec<&str> = result.iter().map(|s| s.id()).collect();
    // Expected order: third party first alphabetically (tp_a, tp_z), then stock alphabetically (stock_a, stock_z)
    assert_eq!(ids, vec!["tp_a", "tp_z", "stock_a", "stock_z"]);

    Ok(())
  }
}
