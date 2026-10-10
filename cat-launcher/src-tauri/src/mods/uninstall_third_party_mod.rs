use std::io;
use std::path::Path;

use crate::filesystem::paths::{
  GetUserGameDataDirError, get_or_create_user_game_data_dir,
};
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyModError {
  #[error("failed to remove installed mod from repository: {0}")]
  Repository(#[from] InstalledModsRepositoryError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to delete mod directory: {0}")]
  DeleteModDirectory(#[from] io::Error),
}

pub async fn uninstall_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  repository: &impl InstalledModsRepository,
) -> Result<(), UninstallThirdPartyModError> {
  repository
    .delete_installed_mod(mod_id, game_variant)
    .await?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let mod_dir = user_game_data_dir.join("mods").join(mod_id);
  tokio::fs::remove_dir_all(&mod_dir).await?;

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
  use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
  use tempfile::tempdir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_uninstall_third_party_mod_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "test-uninstall-mod";

    repo.add_installed_mod(mod_id, &variant).await?;

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let mod_dir = user_data.join("mods").join(mod_id);
    tokio::fs::create_dir_all(&mod_dir).await?;
    tokio::fs::write(mod_dir.join("modinfo.json"), b"{}").await?;

    assert!(mod_dir.exists());
    assert!(repo.is_mod_installed(mod_id, &variant).await?);

    uninstall_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await?;

    assert!(!mod_dir.exists());
    assert!(!repo.is_mod_installed(mod_id, &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_mod_not_installed_in_repo()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "uninstalled-mod";

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let mod_dir = user_data.join("mods").join(mod_id);
    tokio::fs::create_dir_all(&mod_dir).await?;

    let result = uninstall_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    match result {
      Err(UninstallThirdPartyModError::Repository(
        InstalledModsRepositoryError::NotFound(id, var),
      )) => {
        assert_eq!(id, mod_id);
        assert_eq!(var, variant.to_string());
      }
      other => {
        return Err(
          format!("expected NotFound error, got: {:?}", other).into(),
        );
      }
    }

    // Directory should not have been removed since repo failed first
    assert!(mod_dir.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_mod_missing_dir_on_disk()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "ghost-mod";

    repo.add_installed_mod(mod_id, &variant).await?;

    let result = uninstall_third_party_mod(
      mod_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(UninstallThirdPartyModError::DeleteModDirectory(_))
    ));
    assert!(!repo.is_mod_installed(mod_id, &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_mod_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let mod_id = "dda-mod";

    repo
      .add_installed_mod(mod_id, &GameVariant::DarkDaysAhead)
      .await?;

    let user_data = get_or_create_user_game_data_dir(
      &GameVariant::DarkDaysAhead,
      temp_data.path(),
    )
    .await?;
    let mod_dir = user_data.join("mods").join(mod_id);
    tokio::fs::create_dir_all(&mod_dir).await?;

    // Attempt to uninstall for BrightNights
    let result = uninstall_third_party_mod(
      mod_id,
      &GameVariant::BrightNights,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(UninstallThirdPartyModError::Repository(
        InstalledModsRepositoryError::NotFound(_, _)
      ))
    ));

    // Mod directory and repo entry in DDA remain intact
    assert!(mod_dir.exists());
    assert!(
      repo
        .is_mod_installed(mod_id, &GameVariant::DarkDaysAhead)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_mod_all_variants() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let mod_id = format!("mod-{}", variant);
      repo.add_installed_mod(&mod_id, &variant).await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let mod_dir = user_data.join("mods").join(&mod_id);
      tokio::fs::create_dir_all(&mod_dir).await?;

      assert!(mod_dir.exists());
      assert!(repo.is_mod_installed(&mod_id, &variant).await?);

      uninstall_third_party_mod(
        &mod_id,
        &variant,
        temp_data.path(),
        &repo,
      )
      .await?;

      assert!(!mod_dir.exists());
      assert!(!repo.is_mod_installed(&mod_id, &variant).await?);
    }

    Ok(())
  }
}
