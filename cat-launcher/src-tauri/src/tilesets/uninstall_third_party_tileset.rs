use std::io;
use std::path::Path;

use crate::filesystem::paths::{
  GetUserGameDataDirError, get_or_create_user_game_data_dir,
};
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyTilesetError {
  #[error("failed to remove installed tileset from repository: {0}")]
  Repository(#[from] InstalledTilesetsRepositoryError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to delete tileset directory: {0}")]
  DeleteTilesetDirectory(#[from] io::Error),
}

pub async fn uninstall_third_party_tileset(
  tileset_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  repository: &impl InstalledTilesetsRepository,
) -> Result<(), UninstallThirdPartyTilesetError> {
  repository
    .delete_installed_tileset(tileset_id, game_variant)
    .await?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let tileset_dir = user_game_data_dir.join("gfx").join(tileset_id);
  tokio::fs::remove_dir_all(&tileset_dir).await?;

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
  use strum::IntoEnumIterator;
  use tempfile::tempdir;

  use crate::infra::testing::test_database::TestDatabase;
  use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_uninstall_third_party_tileset_success() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "test-tileset";

    repo.add_installed_tileset(tileset_id, &variant).await?;

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let tileset_dir = user_data.join("gfx").join(tileset_id);
    tokio::fs::create_dir_all(&tileset_dir).await?;
    tokio::fs::write(
      tileset_dir.join("tileset.txt"),
      b"NAME:test\nVIEW:Test",
    )
    .await?;

    assert!(tileset_dir.exists());
    assert!(repo.is_tileset_installed(tileset_id, &variant).await?);

    uninstall_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await?;

    assert!(!tileset_dir.exists());
    assert!(!repo.is_tileset_installed(tileset_id, &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_tileset_not_installed_in_repo()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "uninstalled-tileset";

    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let tileset_dir = user_data.join("gfx").join(tileset_id);
    tokio::fs::create_dir_all(&tileset_dir).await?;

    let result = uninstall_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    match result {
      Err(UninstallThirdPartyTilesetError::Repository(
        InstalledTilesetsRepositoryError::NotFound(id, var),
      )) => {
        assert_eq!(id, tileset_id);
        assert_eq!(var, variant.to_string());
      }
      other => {
        return Err(
          format!("expected NotFound error, got: {:?}", other).into(),
        );
      }
    }

    // Directory should not have been removed since repo failed first
    assert!(tileset_dir.exists());

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_tileset_missing_dir_on_disk()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "ghost-tileset";

    repo.add_installed_tileset(tileset_id, &variant).await?;

    let result = uninstall_third_party_tileset(
      tileset_id,
      &variant,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(UninstallThirdPartyTilesetError::DeleteTilesetDirectory(_))
    ));
    assert!(!repo.is_tileset_installed(tileset_id, &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_tileset_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let tileset_id = "dda-tileset";

    repo
      .add_installed_tileset(tileset_id, &GameVariant::DarkDaysAhead)
      .await?;

    let user_data = get_or_create_user_game_data_dir(
      &GameVariant::DarkDaysAhead,
      temp_data.path(),
    )
    .await?;
    let tileset_dir = user_data.join("gfx").join(tileset_id);
    tokio::fs::create_dir_all(&tileset_dir).await?;

    // Attempt to uninstall for BrightNights
    let result = uninstall_third_party_tileset(
      tileset_id,
      &GameVariant::BrightNights,
      temp_data.path(),
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(UninstallThirdPartyTilesetError::Repository(
        InstalledTilesetsRepositoryError::NotFound(_, _)
      ))
    ));

    // Tileset directory and repo entry in DDA remain intact
    assert!(tileset_dir.exists());
    assert!(
      repo
        .is_tileset_installed(tileset_id, &GameVariant::DarkDaysAhead)
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_uninstall_third_party_tileset_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;

    for variant in GameVariant::iter() {
      let tileset_id = format!("tileset-{}", variant);
      repo.add_installed_tileset(&tileset_id, &variant).await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let tileset_dir = user_data.join("gfx").join(&tileset_id);
      tokio::fs::create_dir_all(&tileset_dir).await?;

      uninstall_third_party_tileset(
        &tileset_id,
        &variant,
        temp_data.path(),
        &repo,
      )
      .await?;

      assert!(!tileset_dir.exists());
      assert!(
        !repo.is_tileset_installed(&tileset_id, &variant).await?
      );
    }

    Ok(())
  }
}
