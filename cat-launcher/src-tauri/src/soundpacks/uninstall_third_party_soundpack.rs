use std::io;
use std::path::Path;

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartySoundpackError {
  #[error(
    "failed to remove installed soundpack from repository: {0}"
  )]
  Repository(#[from] InstalledSoundpacksRepositoryError),
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
  #[error("failed to delete soundpack directory: {0}")]
  DeleteSoundpackDirectory(#[from] io::Error),
}

pub async fn uninstall_third_party_soundpack(
  soundpack_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  repository: &impl InstalledSoundpacksRepository,
) -> Result<(), UninstallThirdPartySoundpackError> {
  repository
    .delete_installed_soundpack(soundpack_id, game_variant)
    .await?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let soundpack_dir =
    user_game_data_dir.join("sound").join(soundpack_id);
  tokio::fs::remove_dir_all(&soundpack_dir).await?;

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
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
  use tempfile::tempdir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_uninstall_third_party_soundpack_success() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "test-uninstall-pack";

    repo.add_installed_soundpack(soundpack_id, &variant).await?;

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let soundpack_dir = user_data.join("sound").join(soundpack_id);
    tokio::fs::create_dir_all(&soundpack_dir).await?;
    tokio::fs::write(
      soundpack_dir.join("soundpack.txt"),
      b"NAME:test\nVIEW:Test",
    )
    .await?;

    assert!(soundpack_dir.exists());
    assert!(
      repo.is_soundpack_installed(soundpack_id, &variant).await?
    );

    uninstall_third_party_soundpack(
      soundpack_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await?;

    assert!(!soundpack_dir.exists());
    assert!(
      !repo.is_soundpack_installed(soundpack_id, &variant).await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_soundpack_not_installed_in_repo()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "uninstalled-pack";

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let soundpack_dir = user_data.join("sound").join(soundpack_id);
    tokio::fs::create_dir_all(&soundpack_dir).await?;

    let result = uninstall_third_party_soundpack(
      soundpack_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    match result {
      Err(UninstallThirdPartySoundpackError::Repository(
        InstalledSoundpacksRepositoryError::NotFound(id, var),
      )) => {
        assert_eq!(id, soundpack_id);
        assert_eq!(var, variant.to_string());
      }
      other => {
        return Err(
          format!("expected NotFound error, got: {:?}", other).into(),
        );
      }
    }

    // Directory should not have been removed since repo failed first
    assert!(soundpack_dir.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_soundpack_missing_dir_on_disk()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "ghost-pack";

    repo.add_installed_soundpack(soundpack_id, &variant).await?;

    let result = uninstall_third_party_soundpack(
      soundpack_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(
        UninstallThirdPartySoundpackError::DeleteSoundpackDirectory(
          _
        )
      )
    ));
    assert!(
      !repo.is_soundpack_installed(soundpack_id, &variant).await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_soundpack_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let soundpack_id = "dda-pack";

    repo
      .add_installed_soundpack(
        soundpack_id,
        &GameVariant::DarkDaysAhead,
      )
      .await?;

    let user_data = get_or_create_user_game_data_dir(
      &GameVariant::DarkDaysAhead,
      temp_data.path(),
    )
    .await?;
    let soundpack_dir = user_data.join("sound").join(soundpack_id);
    tokio::fs::create_dir_all(&soundpack_dir).await?;

    // Attempt to uninstall for BrightNights
    let result = uninstall_third_party_soundpack(
      soundpack_id,
      &GameVariant::BrightNights,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(UninstallThirdPartySoundpackError::Repository(
        InstalledSoundpacksRepositoryError::NotFound(_, _)
      ))
    ));

    // Soundpack directory and repo entry in DDA remain intact
    assert!(soundpack_dir.exists());
    assert!(
      repo
        .is_soundpack_installed(
          soundpack_id,
          &GameVariant::DarkDaysAhead
        )
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_soundpack_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let temp_data = tempdir()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let soundpack_id = format!("pack-{}", variant);
      repo
        .add_installed_soundpack(&soundpack_id, &variant)
        .await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let soundpack_dir = user_data.join("sound").join(&soundpack_id);
      tokio::fs::create_dir_all(&soundpack_dir).await?;

      assert!(soundpack_dir.exists());
      assert!(
        repo.is_soundpack_installed(&soundpack_id, &variant).await?
      );

      uninstall_third_party_soundpack(
        &soundpack_id,
        &variant,
        temp_data.path(),
        &repo,
      )
      .await?;

      assert!(!soundpack_dir.exists());
      assert!(
        !repo.is_soundpack_installed(&soundpack_id, &variant).await?
      );
    }

    Ok(())
  }
}
