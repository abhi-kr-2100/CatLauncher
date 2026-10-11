use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs::{create_dir_all, read_to_string};

use crate::filesystem::paths::{
  GetOrCreateDirectoryError, GetUserGameDataDirError,
  get_or_create_directory, get_or_create_user_game_data_dir,
};
use crate::filesystem::utils::{CopyDirError, copy_dir_all};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::download::{DownloadFileError, Downloader};

use crate::infra::utils::OS;
use crate::tilesets::paths::get_tilesets_resource_path;
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::tilesets::types::ThirdPartyTileset;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyTilesetError {
  #[error("failed to get tileset from tilesets.json: {0}")]
  GetTilesetFromJson(#[from] GetTilesetFromJsonError),

  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),

  #[error("failed to download tileset: {0}")]
  Download(#[from] DownloadFileError),

  #[error("failed to extract tileset: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to get tileset parent dir: {0}")]
  GetTilesetParentDir(#[from] GetTilesetParentDirError),

  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get user tileset data dir: {0}")]
  GetUserTilesetDataDir(#[from] GetOrCreateDirectoryError),

  #[error("failed to copy tileset: {0}")]
  Copy(#[from] CopyDirError),

  #[error("failed to update repository: {0}")]
  UpdateRepository(#[from] InstalledTilesetsRepositoryError),
}

#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_tileset(
  tileset_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  repository: &impl InstalledTilesetsRepository,
  reporter: Arc<dyn Reporter + Send + Sync>,
) -> Result<(), InstallThirdPartyTilesetError> {
  let tileset_details =
    get_tileset_from_json(game_variant, tileset_id, resource_dir)
      .await?;

  let tileset_temp_dir = temp_dir
    .join("cat-launcher-tileset-install-dir")
    .join(tileset_id);
  create_dir_all(&tileset_temp_dir).await?;

  let downloaded_file = downloader
    .download_file(
      &tileset_details.installation.download_url,
      &tileset_temp_dir,
      reporter,
    )
    .await?;

  let extraction_dir = tileset_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  let tileset_parent_dir = get_tileset_parent_dir(
    &extraction_dir,
    &tileset_details.installation.tileset,
  )?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let gfx_dir =
    get_or_create_directory(&user_game_data_dir, "gfx").await?;

  let tileset_install_dir = gfx_dir.join(tileset_id);
  copy_dir_all(&tileset_parent_dir, &tileset_install_dir, os).await?;

  let _ = tokio::fs::remove_dir_all(&tileset_temp_dir).await;

  repository
    .add_installed_tileset(tileset_id, game_variant)
    .await?;

  Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetTilesetFromJsonError {
  #[error("failed to read tilesets.json: {0}")]
  ReadTilesetsJson(#[from] std::io::Error),

  #[error("failed to parse tilesets.json: {0}")]
  ParseTilesetsJson(#[from] serde_json::Error),

  #[error("no tilesets found for variant {0}")]
  NoTilesetsForVariant(GameVariant),

  #[error("tileset with id {0} not found")]
  TilesetNotFound(String),
}

async fn get_tileset_from_json(
  game_variant: &GameVariant,
  tileset_id: &str,
  resource_dir: &Path,
) -> Result<ThirdPartyTileset, GetTilesetFromJsonError> {
  let tilesets_json_path = get_tilesets_resource_path(resource_dir);
  let content = read_to_string(&tilesets_json_path).await?;

  let tilesets_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_tilesets = tilesets_data.get(game_variant).ok_or(
    GetTilesetFromJsonError::NoTilesetsForVariant(*game_variant),
  )?;

  let tileset_data = variant_tilesets.get(tileset_id).ok_or(
    GetTilesetFromJsonError::TilesetNotFound(tileset_id.to_string()),
  )?;

  let third_party_tileset = serde_json::from_value::<
    ThirdPartyTileset,
  >(tileset_data.clone())?;

  Ok(third_party_tileset)
}

#[derive(Debug, thiserror::Error)]
pub enum GetTilesetParentDirError {
  #[error("failed to get parent directory for tileset path")]
  ParentDirNotFound,
}

fn get_tileset_parent_dir(
  extracted_dir: &Path,
  tileset_relative_path: &str,
) -> Result<PathBuf, GetTilesetParentDirError> {
  let tileset_path = extracted_dir.join(tileset_relative_path);

  tileset_path
    .parent()
    .ok_or(GetTilesetParentDirError::ParentDirNotFound)
    .map(|p| p.to_path_buf())
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
  use std::num::NonZeroU16;
  use std::sync::Arc;

  use github_mock_api::{Asset, MockServer};
  use strum::IntoEnumIterator;
  use tempfile::tempdir;

  use crate::infra::http_client::ReqwestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::testing::test_zip::create_test_zip;
  use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  struct DummyReporter;
  impl Reporter for DummyReporter {
    fn setup(&self, _max_progress: Option<u64>, _message: &str) {}
    fn progress(&self, _current: u64) {}
    fn set_message(&self, _message: &str) {}
    fn done(&self) {}
  }

  fn create_test_downloader() -> Downloader {
    let client = ReqwestHttpClient::new(reqwest::Client::new());
    Downloader::new(
      client,
      NonZeroU16::new(1).expect("non zero parallel requests"),
    )
  }

  async fn write_tilesets_resource(
    resource_dir: &Path,
    variant: &GameVariant,
    tileset_id: &str,
    download_url: &str,
    tileset_file: &str,
  ) -> TestResult {
    let content_dir = resource_dir.join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      (variant.to_string()): {
        (tileset_id): {
          "id": tileset_id,
          "name": format!("Name for {tileset_id}"),
          "installation": {
            "download_url": download_url,
            "tileset": tileset_file,
          },
          "activity": {
            "activity_type": "github",
            "github": null,
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      serde_json::to_string(&json)?,
    )
    .await?;
    Ok(())
  }

  #[test]
  fn test_get_tileset_parent_dir_unit() {
    let base = Path::new("extracted_dir");
    assert_eq!(
      get_tileset_parent_dir(base, "tileset.txt").unwrap(),
      base
    );

    let nested = Path::new("pack_sub").join("tileset.txt");
    let nested_str = nested.to_str().unwrap();
    assert_eq!(
      get_tileset_parent_dir(base, nested_str).unwrap(),
      base.join("pack_sub")
    );

    let deep = Path::new("a").join("b").join("c").join("tileset.txt");
    let deep_str = deep.to_str().unwrap();
    assert_eq!(
      get_tileset_parent_dir(base, deep_str).unwrap(),
      base.join("a").join("b").join("c")
    );
  }

  #[tokio::test]
  async fn test_get_tileset_from_json_success() -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "test-pack";
    write_tilesets_resource(
      temp.path(),
      &variant,
      tileset_id,
      "https://example.com/test.zip",
      "tileset.txt",
    )
    .await?;

    let tileset =
      get_tileset_from_json(&variant, tileset_id, temp.path())
        .await?;
    assert_eq!(tileset.id, tileset_id);
    assert_eq!(tileset.name, "Name for test-pack");
    assert_eq!(
      tileset.installation.download_url,
      "https://example.com/test.zip"
    );
    assert_eq!(tileset.installation.tileset, "tileset.txt");

    Ok(())
  }

  #[tokio::test]
  async fn test_get_tileset_from_json_no_variant() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      r#"{"BrightNights": {}}"#,
    )
    .await?;

    let result = get_tileset_from_json(
      &GameVariant::DarkDaysAhead,
      "any",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetTilesetFromJsonError::NoTilesetsForVariant(
        GameVariant::DarkDaysAhead
      ))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_get_tileset_from_json_not_found() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      r#"{"DarkDaysAhead": {}}"#,
    )
    .await?;

    let result = get_tileset_from_json(
      &GameVariant::DarkDaysAhead,
      "missing",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetTilesetFromJsonError::TilesetNotFound(id)) if id == "missing"
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_get_tileset_from_json_invalid_json() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      "invalid json data",
    )
    .await?;

    let result = get_tileset_from_json(
      &GameVariant::DarkDaysAhead,
      "any",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetTilesetFromJsonError::ParseTilesetsJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "test-flat-tileset";

    let zip_bytes = create_test_zip(&[
      ("tileset.txt", b"NAME:test-flat\nVIEW:Test Flat Tileset"),
      ("tile.png", b"dummy image content"),
    ])?;

    let asset = Asset::from_bytes(
      "test-flat.zip",
      zip_bytes,
      "application/zip",
    );
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/test-flat.zip",
      server.uri()
    );

    write_tilesets_resource(
      temp_res.path(),
      &variant,
      tileset_id,
      &download_url,
      "tileset.txt",
    )
    .await?;

    install_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      temp_res.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      reporter,
    )
    .await?;

    assert!(
      installed_repo
        .is_tileset_installed(tileset_id, &variant)
        .await?
    );

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_dir = user_data.join("gfx").join(tileset_id);
    assert!(installed_dir.join("tileset.txt").exists());
    assert!(installed_dir.join("tile.png").exists());

    let content =
      tokio::fs::read_to_string(installed_dir.join("tile.png"))
        .await?;
    assert_eq!(content, "dummy image content");

    // Cleaned up temp dir
    let install_temp = temp_dir
      .path()
      .join("cat-launcher-tileset-install-dir")
      .join(tileset_id);
    assert!(!install_temp.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_nested_structure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "test-nested-tileset";

    let zip_bytes = create_test_zip(&[
      (
        "nested_folder/tileset.txt",
        b"NAME:test-nested\nVIEW:Test Nested",
      ),
      ("nested_folder/nested_tile.png", b"nested image bytes"),
    ])?;

    let asset = Asset::from_bytes(
      "nested-pack.zip",
      zip_bytes,
      "application/zip",
    );
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/nested-pack.zip",
      server.uri()
    );

    write_tilesets_resource(
      temp_res.path(),
      &variant,
      tileset_id,
      &download_url,
      "nested_folder/tileset.txt",
    )
    .await?;

    install_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      temp_res.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      reporter,
    )
    .await?;

    assert!(
      installed_repo
        .is_tileset_installed(tileset_id, &variant)
        .await?
    );

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_dir = user_data.join("gfx").join(tileset_id);
    assert!(installed_dir.join("tileset.txt").exists());
    assert!(installed_dir.join("nested_tile.png").exists());

    let content = tokio::fs::read_to_string(
      installed_dir.join("nested_tile.png"),
    )
    .await?;
    assert_eq!(content, "nested image bytes");

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_not_in_json() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("tilesets.json"),
      r#"{"DarkDaysAhead": {}}"#,
    )
    .await?;

    let result = install_third_party_tileset(
      "missing-tileset",
      &variant,
      temp_data.path(),
      temp_res.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyTilesetError::GetTilesetFromJson(
        GetTilesetFromJsonError::TilesetNotFound(_)
      ))
    ));
    assert!(
      !installed_repo
        .is_tileset_installed("missing-tileset", &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_download_failure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "fail-dl-pack";

    write_tilesets_resource(
      temp_res.path(),
      &variant,
      tileset_id,
      "http://127.0.0.1:9/bad.zip",
      "tileset.txt",
    )
    .await?;

    let result = install_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      temp_res.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyTilesetError::Download(_))
    ));
    assert!(
      !installed_repo
        .is_tileset_installed(tileset_id, &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_corrupt_archive()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "corrupt-pack";

    let corrupt_bytes = b"not a zip file at all".to_vec();
    let asset = Asset::from_bytes(
      "corrupt.zip",
      corrupt_bytes,
      "application/zip",
    );
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/corrupt.zip",
      server.uri()
    );

    write_tilesets_resource(
      temp_res.path(),
      &variant,
      tileset_id,
      &download_url,
      "tileset.txt",
    )
    .await?;

    let result = install_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      temp_res.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyTilesetError::Extract(_))
    ));
    assert!(
      !installed_repo
        .is_tileset_installed(tileset_id, &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_tileset_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let os = OS::Linux;

    let zip_bytes = create_test_zip(&[(
      "tileset.txt",
      b"NAME:var-pack\nVIEW:Variant Pack",
    )])?;

    let asset =
      Asset::from_bytes("var-pack.zip", zip_bytes, "application/zip");
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/var-pack.zip",
      server.uri()
    );

    for variant in GameVariant::iter() {
      let tileset_id = format!("tileset-{}", variant);
      write_tilesets_resource(
        temp_res.path(),
        &variant,
        &tileset_id,
        &download_url,
        "tileset.txt",
      )
      .await?;

      install_third_party_tileset(
        &tileset_id,
        &variant,
        temp_data.path(),
        temp_res.path(),
        temp_dir.path(),
        &os,
        &downloader,
        &installed_repo,
        Arc::new(DummyReporter),
      )
      .await?;

      assert!(
        installed_repo
          .is_tileset_installed(&tileset_id, &variant)
          .await?
      );

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      assert!(
        user_data
          .join("gfx")
          .join(&tileset_id)
          .join("tileset.txt")
          .exists()
      );
    }

    Ok(())
  }
}
