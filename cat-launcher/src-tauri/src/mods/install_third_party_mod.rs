use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs::create_dir_all;

use crate::filesystem::paths::{
  GetOrCreateDirectoryError, GetUserGameDataDirError,
  get_or_create_directory, get_or_create_user_game_data_dir,
};
use crate::filesystem::utils::{CopyDirError, copy_dir_all};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::download::{DownloadFileError, Downloader};
use crate::infra::utils::OS;
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError, ModsRepository,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyModError {
  #[error("failed to get mod from repository: {0}")]
  GetModFromRepository(#[from] GetThirdPartyModByIdError),

  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),

  #[error("failed to download mod: {0}")]
  Download(#[from] DownloadFileError),

  #[error("failed to extract mod: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to get mod parent dir: {0}")]
  GetModParentDir(#[from] GetModParentDirError),

  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get user mod data dir: {0}")]
  GetUserModDataDir(#[from] GetOrCreateDirectoryError),

  #[error("failed to copy mod: {0}")]
  Copy(#[from] CopyDirError),

  #[error("failed to update repository: {0}")]
  UpdateRepository(#[from] InstalledModsRepositoryError),
}

#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  installed_mods_repository: &impl InstalledModsRepository,
  mods_repository: &impl ModsRepository,
  reporter: Arc<dyn Reporter + Send + Sync>,
) -> Result<(), InstallThirdPartyModError> {
  let mod_details = mods_repository
    .get_third_party_mod_by_id(mod_id, game_variant)
    .await?;

  let mod_temp_dir =
    temp_dir.join("cat-launcher-mod-install-dir").join(mod_id);
  create_dir_all(&mod_temp_dir).await?;

  let downloaded_file = downloader
    .download_file(
      &mod_details.installation.download_url,
      &mod_temp_dir,
      reporter,
    )
    .await?;

  let extraction_dir = mod_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  let mod_parent_dir = get_mod_parent_dir(
    &extraction_dir,
    &mod_details.installation.modinfo,
  )?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let mods_dir =
    get_or_create_directory(&user_game_data_dir, "mods").await?;

  let mod_install_dir = mods_dir.join(mod_id);
  copy_dir_all(&mod_parent_dir, &mod_install_dir, os).await?;

  let _ = tokio::fs::remove_dir_all(&mod_temp_dir).await;

  installed_mods_repository
    .add_installed_mod(mod_id, game_variant)
    .await?;

  Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetModParentDirError {
  #[error("failed to get parent directory for modinfo path")]
  ParentDirNotFound,
}

fn get_mod_parent_dir(
  extracted_dir: &Path,
  modinfo_relative_path: &str,
) -> Result<PathBuf, GetModParentDirError> {
  let modinfo_path = extracted_dir.join(modinfo_relative_path);

  modinfo_path
    .parent()
    .ok_or(GetModParentDirError::ParentDirNotFound)
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
  use tempfile::tempdir;

  use crate::infra::http_client::ReqwestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::testing::test_zip::create_test_zip;
  use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
  use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
  use crate::mods::types::{ModInstallation, ThirdPartyMod};

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

  fn create_third_party_mod(
    id: &str,
    download_url: &str,
    modinfo_path: &str,
  ) -> ThirdPartyMod {
    ThirdPartyMod {
      id: id.to_string(),
      name: format!("Mod {id}"),
      description: format!("Description for {id}"),
      category: "Content".to_string(),
      installation: ModInstallation {
        download_url: download_url.to_string(),
        modinfo: modinfo_path.to_string(),
      },
      activity: None,
    }
  }

  #[tokio::test]
  async fn test_install_third_party_mod_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "test-flat-mod";

    let zip_bytes = create_test_zip(&[
      (
        "modinfo.json",
        br#"[{"type": "MOD_INFO", "id": "test-flat-mod", "name": "Test Flat Mod"}]"#,
      ),
      ("data.txt", b"mod data content"),
    ])?;

    let asset = Asset::from_bytes(
      "test-flat-mod.zip",
      zip_bytes,
      "application/zip",
    );
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/test-flat-mod.zip",
      server.uri()
    );

    let test_mod =
      create_third_party_mod(mod_id, &download_url, "modinfo.json");
    mods_repo
      .save_third_party_mods(&variant, vec![test_mod])
      .await?;

    install_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      &mods_repo,
      reporter,
    )
    .await?;

    assert!(installed_repo.is_mod_installed(mod_id, &variant).await?);

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_mod_dir = user_data.join("mods").join(mod_id);
    assert!(installed_mod_dir.join("modinfo.json").exists());
    assert!(installed_mod_dir.join("data.txt").exists());

    let content =
      tokio::fs::read_to_string(installed_mod_dir.join("data.txt"))
        .await?;
    assert_eq!(content, "mod data content");

    // Cleaned up temp dir
    let install_temp = temp_dir
      .path()
      .join("cat-launcher-mod-install-dir")
      .join(mod_id);
    assert!(!install_temp.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_mod_nested_structure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "test-nested-mod";

    let zip_bytes = create_test_zip(&[
      (
        "nested_folder/modinfo.json",
        br#"[{"type": "MOD_INFO", "id": "nested", "name": "Nested Mod"}]"#,
      ),
      ("nested_folder/nested.txt", b"nested content"),
    ])?;

    let asset = Asset::from_bytes(
      "nested-mod.zip",
      zip_bytes,
      "application/zip",
    );
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/nested-mod.zip",
      server.uri()
    );

    let test_mod = create_third_party_mod(
      mod_id,
      &download_url,
      "nested_folder/modinfo.json",
    );
    mods_repo
      .save_third_party_mods(&variant, vec![test_mod])
      .await?;

    install_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      &mods_repo,
      reporter,
    )
    .await?;

    assert!(installed_repo.is_mod_installed(mod_id, &variant).await?);

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_mod_dir = user_data.join("mods").join(mod_id);
    assert!(installed_mod_dir.join("modinfo.json").exists());
    assert!(installed_mod_dir.join("nested.txt").exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_mod_not_in_repository()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    let result = install_third_party_mod(
      "missing-mod",
      &variant,
      temp_data.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      &mods_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyModError::GetModFromRepository(_))
    ));
    assert!(
      !installed_repo
        .is_mod_installed("missing-mod", &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_mod_download_failure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "fail-dl-mod";

    // Non-existent server port
    let test_mod = create_third_party_mod(
      mod_id,
      "http://127.0.0.1:9/bad.zip",
      "modinfo.json",
    );
    mods_repo
      .save_third_party_mods(&variant, vec![test_mod])
      .await?;

    let result = install_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      &mods_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyModError::Download(_))
    ));
    assert!(
      !installed_repo.is_mod_installed(mod_id, &variant).await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_mod_corrupt_archive() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "corrupt-mod";

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

    let test_mod =
      create_third_party_mod(mod_id, &download_url, "modinfo.json");
    mods_repo
      .save_third_party_mods(&variant, vec![test_mod])
      .await?;

    let result = install_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      temp_dir.path(),
      &os,
      &downloader,
      &installed_repo,
      &mods_repo,
      reporter,
    )
    .await;

    assert!(matches!(
      result,
      Err(InstallThirdPartyModError::Extract(_))
    ));
    assert!(
      !installed_repo.is_mod_installed(mod_id, &variant).await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_mod_all_variants() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_dir = tempdir()?;
    let os = OS::Linux;

    let zip_bytes = create_test_zip(&[(
      "modinfo.json",
      br#"[{"type": "MOD_INFO", "id": "var-mod", "name": "Variant Mod"}]"#,
    )])?;

    let asset =
      Asset::from_bytes("var-mod.zip", zip_bytes, "application/zip");
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/var-mod.zip",
      server.uri()
    );

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let mod_id = format!("mod-{}", variant);
      let test_mod = create_third_party_mod(
        &mod_id,
        &download_url,
        "modinfo.json",
      );
      mods_repo
        .save_third_party_mods(&variant, vec![test_mod])
        .await?;

      install_third_party_mod(
        &mod_id,
        &variant,
        temp_data.path(),
        temp_dir.path(),
        &os,
        &downloader,
        &installed_repo,
        &mods_repo,
        Arc::new(DummyReporter),
      )
      .await?;

      assert!(
        installed_repo.is_mod_installed(&mod_id, &variant).await?
      );

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      assert!(
        user_data
          .join("mods")
          .join(&mod_id)
          .join("modinfo.json")
          .exists()
      );
    }

    Ok(())
  }

  #[test]
  fn test_get_mod_parent_dir_unit() {
    let base = Path::new("test_extracted");
    assert_eq!(
      get_mod_parent_dir(base, "modinfo.json").unwrap(),
      base
    );
    let nested_path = Path::new("nested").join("modinfo.json");
    let nested_str = nested_path.to_str().unwrap();
    assert_eq!(
      get_mod_parent_dir(base, nested_str).unwrap(),
      base.join("nested")
    );
    let deep_path =
      Path::new("a").join("b").join("c").join("modinfo.json");
    let deep_str = deep_path.to_str().unwrap();
    assert_eq!(
      get_mod_parent_dir(base, deep_str).unwrap(),
      base.join("a").join("b").join("c")
    );
  }
}
