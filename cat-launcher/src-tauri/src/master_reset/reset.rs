use std::path::Path;

use tokio::fs;

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::soundpacks::repository::installed_soundpacks_repository::{
  InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum MasterResetError {
  #[error("failed to get user data directory: {0}")]
  UserDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to read directory: {0}")]
  ReadDir(#[from] std::io::Error),

  #[error("failed to remove entry: {0}")]
  RemoveEntry(std::io::Error),

  #[error("failed to delete installed mods: {0}")]
  DeleteMods(#[from] InstalledModsRepositoryError),

  #[error("failed to delete installed soundpacks: {0}")]
  DeleteSoundpacks(#[from] InstalledSoundpacksRepositoryError),

  #[error("failed to delete installed tilesets: {0}")]
  DeleteTilesets(#[from] InstalledTilesetsRepositoryError),
}

async fn should_skip(
  entry: &fs::DirEntry,
) -> Result<bool, std::io::Error> {
  let file_name = entry.file_name();
  let file_name_str = file_name.to_string_lossy();

  if file_name_str.eq_ignore_ascii_case("save")
    && entry.file_type().await?.is_dir()
  {
    return Ok(true);
  }

  Ok(false)
}

pub async fn master_reset(
  variant: &GameVariant,
  data_dir: &Path,
  installed_mods_repository: &impl InstalledModsRepository,
  installed_soundpacks_repository: &impl InstalledSoundpacksRepository,
  installed_tilesets_repository: &impl InstalledTilesetsRepository,
) -> Result<(), MasterResetError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;

  let mut entries = fs::read_dir(&user_data_dir).await?;

  while let Some(entry) = entries.next_entry().await? {
    if should_skip(&entry).await? {
      continue;
    }

    let path = entry.path();
    if entry.file_type().await?.is_dir() {
      fs::remove_dir_all(&path)
        .await
        .map_err(MasterResetError::RemoveEntry)?;
    } else {
      fs::remove_file(&path)
        .await
        .map_err(MasterResetError::RemoveEntry)?;
    }
  }

  // Update DB sequentially to avoid deadlocks
  installed_mods_repository
    .delete_all_installed_mods(variant)
    .await?;

  installed_soundpacks_repository
    .delete_all_installed_soundpacks(variant)
    .await?;

  installed_tilesets_repository
    .delete_all_installed_tilesets(variant)
    .await?;

  Ok(())
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
  use std::path::PathBuf;

  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
  use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
  use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
  use async_trait::async_trait;
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  async fn seed_user_data(
    data_dir: &Path,
    variant: &GameVariant,
  ) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let user_data =
      get_or_create_user_game_data_dir(variant, data_dir).await?;
    let save_dir = user_data.join("save");
    tokio::fs::create_dir_all(&save_dir).await?;
    tokio::fs::write(save_dir.join("save1.json"), b"saved game")
      .await?;
    let config_dir = user_data.join("config");
    tokio::fs::create_dir_all(&config_dir).await?;
    tokio::fs::write(config_dir.join("opts.txt"), b"options").await?;
    tokio::fs::write(user_data.join("temp.tmp"), b"temp data")
      .await?;
    Ok((save_dir, config_dir))
  }

  fn mock_io_error() -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "mock failure",
    ))
  }

  struct FailingModsRepository {
    fail_delete_all: bool,
  }

  #[async_trait]
  impl InstalledModsRepository for FailingModsRepository {
    async fn add_installed_mod(
      &self,
      _mod_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledModsRepositoryError> {
      Ok(())
    }

    async fn delete_installed_mod(
      &self,
      _mod_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledModsRepositoryError> {
      Ok(())
    }

    async fn delete_all_installed_mods(
      &self,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledModsRepositoryError> {
      if self.fail_delete_all {
        return Err(InstalledModsRepositoryError::DeleteAll(
          mock_io_error(),
        ));
      }
      Ok(())
    }

    async fn is_mod_installed(
      &self,
      _mod_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<bool, InstalledModsRepositoryError> {
      Ok(false)
    }
  }

  struct FailingSoundpacksRepository {
    fail_delete_all: bool,
  }

  #[async_trait]
  impl InstalledSoundpacksRepository for FailingSoundpacksRepository {
    async fn add_installed_soundpack(
      &self,
      _soundpack_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError> {
      Ok(())
    }

    async fn delete_installed_soundpack(
      &self,
      _soundpack_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError> {
      Ok(())
    }

    async fn delete_all_installed_soundpacks(
      &self,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError> {
      if self.fail_delete_all {
        return Err(InstalledSoundpacksRepositoryError::DeleteAll(
          mock_io_error(),
        ));
      }
      Ok(())
    }

    async fn is_soundpack_installed(
      &self,
      _soundpack_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<bool, InstalledSoundpacksRepositoryError> {
      Ok(false)
    }
  }

  struct FailingTilesetsRepository {
    fail_delete_all: bool,
  }

  #[async_trait]
  impl InstalledTilesetsRepository for FailingTilesetsRepository {
    async fn add_installed_tileset(
      &self,
      _tileset_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError> {
      Ok(())
    }

    async fn delete_installed_tileset(
      &self,
      _tileset_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError> {
      Ok(())
    }

    async fn delete_all_installed_tilesets(
      &self,
      _game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError> {
      if self.fail_delete_all {
        return Err(InstalledTilesetsRepositoryError::DeleteAll(
          mock_io_error(),
        ));
      }
      Ok(())
    }

    async fn is_tileset_installed(
      &self,
      _tileset_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<bool, InstalledTilesetsRepositoryError> {
      Ok(false)
    }
  }

  #[tokio::test]
  async fn test_master_reset_preserves_save_dir_and_removes_other_entries()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let (save_dir, config_dir) =
        seed_user_data(temp_data.path(), &variant).await?;

      master_reset(
        &variant,
        temp_data.path(),
        &mods_repo,
        &soundpacks_repo,
        &tilesets_repo,
      )
      .await?;

      assert_eq!(
        tokio::fs::read_to_string(save_dir.join("save1.json"))
          .await?,
        "saved game",
        "save directory content must be preserved"
      );

      assert!(
        !config_dir.exists(),
        "non-save directories should be deleted"
      );
      assert!(
        !save_dir
          .parent()
          .expect("save dir has a parent")
          .join("temp.tmp")
          .exists(),
        "files outside save directory should be deleted"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_clears_installed_content_for_target_variant()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let other_variant = if variant == GameVariant::DarkDaysAhead {
        GameVariant::BrightNights
      } else {
        GameVariant::DarkDaysAhead
      };

      mods_repo.add_installed_mod("mod1", &variant).await?;
      soundpacks_repo
        .add_installed_soundpack("soundpack1", &variant)
        .await?;
      tilesets_repo
        .add_installed_tileset("tileset1", &variant)
        .await?;

      // Seed records for other_variant to verify reset isolates target variant
      mods_repo
        .add_installed_mod("mod_other", &other_variant)
        .await?;
      soundpacks_repo
        .add_installed_soundpack("soundpack_other", &other_variant)
        .await?;
      tilesets_repo
        .add_installed_tileset("tileset_other", &other_variant)
        .await?;

      master_reset(
        &variant,
        temp_data.path(),
        &mods_repo,
        &soundpacks_repo,
        &tilesets_repo,
      )
      .await?;

      assert!(
        !mods_repo.is_mod_installed("mod1", &variant).await?,
        "installed mods should be cleared"
      );
      assert!(
        !soundpacks_repo
          .is_soundpack_installed("soundpack1", &variant)
          .await?,
        "installed soundpacks should be cleared"
      );
      assert!(
        !tilesets_repo
          .is_tileset_installed("tileset1", &variant)
          .await?,
        "installed tilesets should be cleared"
      );

      // Verify other_variant records remain intact
      assert!(
        mods_repo
          .is_mod_installed("mod_other", &other_variant)
          .await?,
        "other variant mods must be preserved"
      );
      assert!(
        soundpacks_repo
          .is_soundpack_installed("soundpack_other", &other_variant)
          .await?,
        "other variant soundpacks must be preserved"
      );
      assert!(
        tilesets_repo
          .is_tileset_installed("tileset_other", &other_variant)
          .await?,
        "other variant tilesets must be preserved"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_succeeds_with_empty_database_and_empty_user_data_dir()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;

    // DB has entries only for another variant; the target variant has none
    mods_repo
      .add_installed_mod("mod_other", &GameVariant::BrightNights)
      .await?;
    soundpacks_repo
      .add_installed_soundpack(
        "soundpack_other",
        &GameVariant::BrightNights,
      )
      .await?;
    tilesets_repo
      .add_installed_tileset(
        "tileset_other",
        &GameVariant::BrightNights,
      )
      .await?;

    master_reset(
      &variant,
      temp_data.path(),
      &mods_repo,
      &soundpacks_repo,
      &tilesets_repo,
    )
    .await?;

    // User data dir was created but must contain nothing
    let user_data =
      temp_data.path().join("UserData").join(variant.id());
    assert!(user_data.is_dir());
    let mut entries = tokio::fs::read_dir(&user_data).await?;
    assert!(
      entries.next_entry().await?.is_none(),
      "user data directory should be empty"
    );

    // Other variants' entries must remain intact
    assert!(
      mods_repo
        .is_mod_installed("mod_other", &GameVariant::BrightNights)
        .await?,
      "other variant mods must be preserved"
    );
    assert!(
      soundpacks_repo
        .is_soundpack_installed(
          "soundpack_other",
          &GameVariant::BrightNights,
        )
        .await?,
      "other variant soundpacks must be preserved"
    );
    assert!(
      tilesets_repo
        .is_tileset_installed(
          "tileset_other",
          &GameVariant::BrightNights,
        )
        .await?,
      "other variant tilesets must be preserved"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_preserves_save_dir_case_insensitively()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;
    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;

    // "SAVE" dir must be skipped case-insensitively
    let save_dir = user_data.join("SAVE");
    tokio::fs::create_dir_all(&save_dir).await?;
    tokio::fs::write(save_dir.join("world.json"), b"world").await?;

    master_reset(
      &variant,
      temp_data.path(),
      &mods_repo,
      &soundpacks_repo,
      &tilesets_repo,
    )
    .await?;

    assert_eq!(
      tokio::fs::read_to_string(save_dir.join("world.json")).await?,
      "world",
      "case-insensitive save directory content must be preserved"
    );

    let mut entries = tokio::fs::read_dir(&user_data).await?;
    let mut names = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
      names.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert_eq!(names, vec!["SAVE".to_string()]);

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_deletes_save_file() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;
    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;

    // A file named "save" is NOT a directory and must be deleted
    tokio::fs::write(user_data.join("save"), b"not a directory")
      .await?;

    master_reset(
      &variant,
      temp_data.path(),
      &mods_repo,
      &soundpacks_repo,
      &tilesets_repo,
    )
    .await?;

    assert!(
      !user_data.join("save").exists(),
      "file named save must be deleted since it is not a directory"
    );

    let mut entries = tokio::fs::read_dir(&user_data).await?;
    assert!(
      entries.next_entry().await?.is_none(),
      "user data directory should be empty"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_partial_failure_when_mods_delete_all_fails()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let failing_mods_repo = FailingModsRepository {
      fail_delete_all: true,
    };

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;
    let (save_dir, config_dir) =
      seed_user_data(temp_data.path(), &variant).await?;

    soundpacks_repo
      .add_installed_soundpack("soundpack1", &variant)
      .await?;
    tilesets_repo
      .add_installed_tileset("tileset1", &variant)
      .await?;

    let result = master_reset(
      &variant,
      temp_data.path(),
      &failing_mods_repo,
      &soundpacks_repo,
      &tilesets_repo,
    )
    .await;

    assert!(
      matches!(result, Err(MasterResetError::DeleteMods(_))),
      "expected mods delete failure, got {result:?}"
    );

    // File deletion happens before DB steps, so it must have completed
    assert!(
      !config_dir.exists(),
      "files should still be deleted before the DB step fails"
    );
    assert_eq!(
      tokio::fs::read_to_string(save_dir.join("save1.json")).await?,
      "saved game",
      "save directory must be preserved on partial failure"
    );

    // Steps after the mods delete must not have run
    assert!(
      soundpacks_repo
        .is_soundpack_installed("soundpack1", &variant)
        .await?,
      "soundpacks should not be deleted when mods step fails"
    );
    assert!(
      tilesets_repo
        .is_tileset_installed("tileset1", &variant)
        .await?,
      "tilesets should not be deleted when mods step fails"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_partial_failure_when_soundpacks_delete_all_fails()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let tilesets_repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let failing_soundpacks_repo = FailingSoundpacksRepository {
      fail_delete_all: true,
    };

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;
    let (save_dir, config_dir) =
      seed_user_data(temp_data.path(), &variant).await?;

    mods_repo.add_installed_mod("mod1", &variant).await?;
    tilesets_repo
      .add_installed_tileset("tileset1", &variant)
      .await?;

    let result = master_reset(
      &variant,
      temp_data.path(),
      &mods_repo,
      &failing_soundpacks_repo,
      &tilesets_repo,
    )
    .await;

    assert!(
      matches!(result, Err(MasterResetError::DeleteSoundpacks(_))),
      "expected soundpacks delete failure, got {result:?}"
    );

    // Steps before the failure must have completed
    assert!(
      !config_dir.exists(),
      "files should still be deleted before the DB step fails"
    );
    assert!(
      !mods_repo.is_mod_installed("mod1", &variant).await?,
      "mods should be deleted before the soundpacks step"
    );

    // Steps after the soundpacks delete must not have run
    assert!(
      tilesets_repo
        .is_tileset_installed("tileset1", &variant)
        .await?,
      "tilesets should not be deleted when soundpacks step fails"
    );
    assert_eq!(
      tokio::fs::read_to_string(save_dir.join("save1.json")).await?,
      "saved game",
      "save directory must be preserved on partial failure"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_master_reset_partial_failure_when_tilesets_delete_all_fails()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let mods_repo =
      SqliteInstalledModsRepository::new(db.pool().clone());
    let soundpacks_repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let failing_tilesets_repo = FailingTilesetsRepository {
      fail_delete_all: true,
    };

    let temp_data = TempDir::new()?;
    let variant = GameVariant::DarkDaysAhead;
    let (save_dir, config_dir) =
      seed_user_data(temp_data.path(), &variant).await?;

    mods_repo.add_installed_mod("mod1", &variant).await?;
    soundpacks_repo
      .add_installed_soundpack("soundpack1", &variant)
      .await?;

    let result = master_reset(
      &variant,
      temp_data.path(),
      &mods_repo,
      &soundpacks_repo,
      &failing_tilesets_repo,
    )
    .await;

    assert!(
      matches!(result, Err(MasterResetError::DeleteTilesets(_))),
      "expected tilesets delete failure, got {result:?}"
    );

    // Steps before the failure must have completed
    assert!(
      !config_dir.exists(),
      "files should still be deleted before the DB step fails"
    );
    assert!(
      !mods_repo.is_mod_installed("mod1", &variant).await?,
      "mods should be deleted before the tilesets step"
    );
    assert!(
      !soundpacks_repo
        .is_soundpack_installed("soundpack1", &variant)
        .await?,
      "soundpacks should be deleted before the tilesets step"
    );
    assert_eq!(
      tokio::fs::read_to_string(save_dir.join("save1.json")).await?,
      "saved game",
      "save directory must be preserved on partial failure"
    );

    Ok(())
  }
}
