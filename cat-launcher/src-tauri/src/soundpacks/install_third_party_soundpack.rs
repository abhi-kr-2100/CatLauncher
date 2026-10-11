use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::fs::{create_dir_all, read_to_string};

use downloader::progress::Reporter;

use crate::filesystem::paths::{
  GetOrCreateDirectoryError, GetUserGameDataDirError,
  get_or_create_directory, get_or_create_user_game_data_dir,
};
use crate::filesystem::utils::{CopyDirError, copy_dir_all};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::download::{DownloadFileError, Downloader};

use crate::infra::utils::OS;
use crate::soundpacks::paths::get_soundpacks_resource_path;
use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::soundpacks::types::ThirdPartySoundpack;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartySoundpackError {
  #[error("failed to get soundpack from soundpacks.json: {0}")]
  GetSoundpackFromJson(#[from] GetSoundpackFromJsonError),

  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),

  #[error("failed to download soundpack: {0}")]
  Download(#[from] DownloadFileError),

  #[error("failed to extract soundpack: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to get soundpack parent dir: {0}")]
  GetSoundpackParentDir(#[from] GetSoundpackParentDirError),

  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get user soundpack data dir: {0}")]
  GetUserSoundpackDataDir(#[from] GetOrCreateDirectoryError),

  #[error("failed to copy soundpack: {0}")]
  Copy(#[from] CopyDirError),

  #[error("failed to update repository: {0}")]
  UpdateRepository(#[from] InstalledSoundpacksRepositoryError),
}

#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_soundpack(
  soundpack_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  repository: &impl InstalledSoundpacksRepository,
  reporter: Arc<dyn Reporter + Send + Sync>,
) -> Result<(), InstallThirdPartySoundpackError> {
  let soundpack_details =
    get_soundpack_from_json(game_variant, soundpack_id, resource_dir)
      .await?;

  let soundpack_temp_dir = temp_dir
    .join("cat-launcher-soundpack-install-dir")
    .join(soundpack_id);
  create_dir_all(&soundpack_temp_dir).await?;

  let downloaded_file = downloader
    .download_file(
      &soundpack_details.installation.download_url,
      &soundpack_temp_dir,
      reporter,
    )
    .await?;

  let extraction_dir = soundpack_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  let soundpack_parent_dir = get_soundpack_parent_dir(
    &extraction_dir,
    &soundpack_details.installation.soundpack,
  )?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let sounds_dir =
    get_or_create_directory(&user_game_data_dir, "sound").await?;

  let soundpack_install_dir = sounds_dir.join(soundpack_id);
  copy_dir_all(&soundpack_parent_dir, &soundpack_install_dir, os)
    .await?;

  repository
    .add_installed_soundpack(soundpack_id, game_variant)
    .await?;

  let _ = tokio::fs::remove_dir_all(&soundpack_temp_dir).await;

  Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetSoundpackFromJsonError {
  #[error("failed to read soundpacks.json: {0}")]
  ReadSoundpacksJson(#[from] std::io::Error),

  #[error("failed to parse soundpacks.json: {0}")]
  ParseSoundpacksJson(#[from] serde_json::Error),

  #[error("no soundpacks found for variant {0}")]
  NoSoundpacksForVariant(GameVariant),

  #[error("soundpack with id {0} not found")]
  SoundpackNotFound(String),
}

async fn get_soundpack_from_json(
  game_variant: &GameVariant,
  soundpack_id: &str,
  resource_dir: &Path,
) -> Result<ThirdPartySoundpack, GetSoundpackFromJsonError> {
  let soundpacks_json_path =
    get_soundpacks_resource_path(resource_dir);
  let content = read_to_string(&soundpacks_json_path).await?;

  let soundpacks_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_soundpacks = soundpacks_data.get(game_variant).ok_or(
    GetSoundpackFromJsonError::NoSoundpacksForVariant(*game_variant),
  )?;

  let soundpack_data = variant_soundpacks.get(soundpack_id).ok_or(
    GetSoundpackFromJsonError::SoundpackNotFound(
      soundpack_id.to_string(),
    ),
  )?;

  let third_party_soundpack = serde_json::from_value::<
    ThirdPartySoundpack,
  >(soundpack_data.clone())?;

  Ok(third_party_soundpack)
}

#[derive(Debug, thiserror::Error)]
pub enum GetSoundpackParentDirError {
  #[error("failed to get parent directory for soundpack path")]
  ParentDirNotFound,
}

fn get_soundpack_parent_dir(
  extracted_dir: &Path,
  soundpack_relative_path: &str,
) -> Result<PathBuf, GetSoundpackParentDirError> {
  let soundpack_path = extracted_dir.join(soundpack_relative_path);

  soundpack_path
    .parent()
    .ok_or(GetSoundpackParentDirError::ParentDirNotFound)
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
  use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;

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

  async fn write_soundpacks_resource(
    resource_dir: &Path,
    variant: &GameVariant,
    soundpack_id: &str,
    download_url: &str,
    soundpack_file: &str,
  ) -> TestResult {
    let content_dir = resource_dir.join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let json = serde_json::json!({
      variant.to_string(): {
        soundpack_id: {
          "id": soundpack_id,
          "name": format!("Name for {soundpack_id}"),
          "installation": {
            "download_url": download_url,
            "soundpack": soundpack_file,
          },
          "activity": {
            "activity_type": "github",
            "github": null,
          }
        }
      }
    });
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      serde_json::to_string(&json)?,
    )
    .await?;
    Ok(())
  }

  #[test]
  fn test_get_soundpack_parent_dir_unit() {
    let base = Path::new("extracted_dir");
    assert_eq!(
      get_soundpack_parent_dir(base, "soundpack.txt").unwrap(),
      base
    );

    let nested = Path::new("pack_sub").join("soundpack.txt");
    let nested_str = nested.to_str().unwrap();
    assert_eq!(
      get_soundpack_parent_dir(base, nested_str).unwrap(),
      base.join("pack_sub")
    );

    let deep =
      Path::new("a").join("b").join("c").join("soundpack.txt");
    let deep_str = deep.to_str().unwrap();
    assert_eq!(
      get_soundpack_parent_dir(base, deep_str).unwrap(),
      base.join("a").join("b").join("c")
    );
  }

  #[tokio::test]
  async fn test_get_soundpack_from_json_success() -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "test-pack";
    write_soundpacks_resource(
      temp.path(),
      &variant,
      soundpack_id,
      "https://example.com/test.zip",
      "soundpack.txt",
    )
    .await?;

    let sp =
      get_soundpack_from_json(&variant, soundpack_id, temp.path())
        .await?;
    assert_eq!(sp.id, soundpack_id);
    assert_eq!(sp.name, "Name for test-pack");
    assert_eq!(
      sp.installation.download_url,
      "https://example.com/test.zip"
    );
    assert_eq!(sp.installation.soundpack, "soundpack.txt");

    Ok(())
  }

  #[tokio::test]
  async fn test_get_soundpack_from_json_no_variant() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      r#"{"BrightNights": {}}"#,
    )
    .await?;

    let result = get_soundpack_from_json(
      &GameVariant::DarkDaysAhead,
      "any",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetSoundpackFromJsonError::NoSoundpacksForVariant(
        GameVariant::DarkDaysAhead
      ))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_get_soundpack_from_json_not_found() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      r#"{"DarkDaysAhead": {}}"#,
    )
    .await?;

    let result = get_soundpack_from_json(
      &GameVariant::DarkDaysAhead,
      "missing",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetSoundpackFromJsonError::SoundpackNotFound(id)) if id == "missing"
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_get_soundpack_from_json_invalid_json() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(
      content_dir.join("soundpacks.json"),
      "invalid json data",
    )
    .await?;

    let result = get_soundpack_from_json(
      &GameVariant::DarkDaysAhead,
      "any",
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(GetSoundpackFromJsonError::ParseSoundpacksJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_success() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "test-flat-soundpack";

    let zip_bytes = create_test_zip(&[
      ("soundpack.txt", b"NAME:test-flat\nVIEW:Test Flat Soundpack"),
      ("sound.ogg", b"dummy audio content"),
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

    write_soundpacks_resource(
      temp_res.path(),
      &variant,
      soundpack_id,
      &download_url,
      "soundpack.txt",
    )
    .await?;

    install_third_party_soundpack(
      soundpack_id,
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
        .is_soundpack_installed(soundpack_id, &variant)
        .await?
    );

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_dir = user_data.join("sound").join(soundpack_id);
    assert!(installed_dir.join("soundpack.txt").exists());
    assert!(installed_dir.join("sound.ogg").exists());

    let content =
      tokio::fs::read_to_string(installed_dir.join("sound.ogg"))
        .await?;
    assert_eq!(content, "dummy audio content");

    // Cleaned up temp dir
    let install_temp = temp_dir
      .path()
      .join("cat-launcher-soundpack-install-dir")
      .join(soundpack_id);
    assert!(!install_temp.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_nested_structure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "test-nested-soundpack";

    let zip_bytes = create_test_zip(&[
      (
        "nested_folder/soundpack.txt",
        b"NAME:test-nested\nVIEW:Test Nested",
      ),
      ("nested_folder/nested_audio.ogg", b"nested audio bytes"),
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

    write_soundpacks_resource(
      temp_res.path(),
      &variant,
      soundpack_id,
      &download_url,
      "nested_folder/soundpack.txt",
    )
    .await?;

    install_third_party_soundpack(
      soundpack_id,
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
        .is_soundpack_installed(soundpack_id, &variant)
        .await?
    );

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let installed_dir = user_data.join("sound").join(soundpack_id);
    assert!(installed_dir.join("soundpack.txt").exists());
    assert!(installed_dir.join("nested_audio.ogg").exists());

    let content = tokio::fs::read_to_string(
      installed_dir.join("nested_audio.ogg"),
    )
    .await?;
    assert_eq!(content, "nested audio bytes");

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_not_in_json()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
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
      content_dir.join("soundpacks.json"),
      r#"{"DarkDaysAhead": {}}"#,
    )
    .await?;

    let result = install_third_party_soundpack(
      "missing-soundpack",
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
      Err(InstallThirdPartySoundpackError::GetSoundpackFromJson(
        GetSoundpackFromJsonError::SoundpackNotFound(_)
      ))
    ));
    assert!(
      !installed_repo
        .is_soundpack_installed("missing-soundpack", &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_download_failure()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "fail-dl-pack";

    write_soundpacks_resource(
      temp_res.path(),
      &variant,
      soundpack_id,
      "http://127.0.0.1:9/bad.zip",
      "soundpack.txt",
    )
    .await?;

    let result = install_third_party_soundpack(
      soundpack_id,
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
      Err(InstallThirdPartySoundpackError::Download(_))
    ));
    assert!(
      !installed_repo
        .is_soundpack_installed(soundpack_id, &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_corrupt_archive()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let reporter = Arc::new(DummyReporter);
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "corrupt-pack";

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

    write_soundpacks_resource(
      temp_res.path(),
      &variant,
      soundpack_id,
      &download_url,
      "soundpack.txt",
    )
    .await?;

    let result = install_third_party_soundpack(
      soundpack_id,
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
      Err(InstallThirdPartySoundpackError::Extract(_))
    ));
    assert!(
      !installed_repo
        .is_soundpack_installed(soundpack_id, &variant)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_install_third_party_soundpack_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;
    let installed_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let downloader = create_test_downloader();
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let temp_dir = tempdir()?;
    let os = OS::Linux;

    let zip_bytes = create_test_zip(&[(
      "soundpack.txt",
      b"NAME:var-pack\nVIEW:Variant Pack",
    )])?;

    let asset =
      Asset::from_bytes("var-pack.zip", zip_bytes, "application/zip");
    server.add_asset("owner", "repo", "v1.0.0", asset).await;

    let download_url = format!(
      "{}/owner/repo/releases/download/v1.0.0/var-pack.zip",
      server.uri()
    );

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let soundpack_id = format!("soundpack-{}", variant);
      write_soundpacks_resource(
        temp_res.path(),
        &variant,
        &soundpack_id,
        &download_url,
        "soundpack.txt",
      )
      .await?;

      install_third_party_soundpack(
        &soundpack_id,
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
          .is_soundpack_installed(&soundpack_id, &variant)
          .await?
      );

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      assert!(
        user_data
          .join("sound")
          .join(&soundpack_id)
          .join("soundpack.txt")
          .exists()
      );
    }

    Ok(())
  }
}
