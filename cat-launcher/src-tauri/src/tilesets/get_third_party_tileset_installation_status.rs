use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::tilesets::types::TilesetInstallationStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyTilesetInstallationStatusError {
  #[error("failed to check tileset installation status: {0}")]
  Repository(#[from] InstalledTilesetsRepositoryError),
}

pub async fn get_third_party_tileset_installation_status(
  tileset_id: &str,
  variant: &GameVariant,
  repository: &impl InstalledTilesetsRepository,
) -> Result<
  TilesetInstallationStatus,
  GetThirdPartyTilesetInstallationStatusError,
> {
  let is_installed =
    repository.is_tileset_installed(tileset_id, variant).await?;

  Ok(if is_installed {
    TilesetInstallationStatus::Installed
  } else {
    TilesetInstallationStatus::NotInstalled
  })
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
  use crate::infra::testing::test_database::TestDatabase;
  use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_installation_status_not_installed() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let status = get_third_party_tileset_installation_status(
      "uninstalled-tileset",
      &variant,
      &repo,
    )
    .await?;
    assert_eq!(status, TilesetInstallationStatus::NotInstalled);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_installed() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "installed-tileset";

    repo.add_installed_tileset(tileset_id, &variant).await?;

    let status = get_third_party_tileset_installation_status(
      tileset_id, &variant, &repo,
    )
    .await?;
    assert_eq!(status, TilesetInstallationStatus::Installed);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_after_uninstalled()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tileset_id = "toggle-tileset";

    repo.add_installed_tileset(tileset_id, &variant).await?;
    assert_eq!(
      get_third_party_tileset_installation_status(
        tileset_id, &variant, &repo
      )
      .await?,
      TilesetInstallationStatus::Installed
    );

    repo.delete_installed_tileset(tileset_id, &variant).await?;
    assert_eq!(
      get_third_party_tileset_installation_status(
        tileset_id, &variant, &repo
      )
      .await?,
      TilesetInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let tileset_id = "isolated-tileset";

    repo
      .add_installed_tileset(tileset_id, &GameVariant::DarkDaysAhead)
      .await?;

    assert_eq!(
      get_third_party_tileset_installation_status(
        tileset_id,
        &GameVariant::DarkDaysAhead,
        &repo
      )
      .await?,
      TilesetInstallationStatus::Installed
    );
    assert_eq!(
      get_third_party_tileset_installation_status(
        tileset_id,
        &GameVariant::BrightNights,
        &repo
      )
      .await?,
      TilesetInstallationStatus::NotInstalled
    );
    assert_eq!(
      get_third_party_tileset_installation_status(
        tileset_id,
        &GameVariant::TheLastGeneration,
        &repo
      )
      .await?,
      TilesetInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_all_variants() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    for variant in GameVariant::iter() {
      let tileset_id = format!("status-tileset-{}", variant);
      assert_eq!(
        get_third_party_tileset_installation_status(
          &tileset_id,
          &variant,
          &repo
        )
        .await?,
        TilesetInstallationStatus::NotInstalled
      );

      repo.add_installed_tileset(&tileset_id, &variant).await?;
      assert_eq!(
        get_third_party_tileset_installation_status(
          &tileset_id,
          &variant,
          &repo
        )
        .await?,
        TilesetInstallationStatus::Installed
      );
    }

    Ok(())
  }

  struct FailingTilesetRepository;

  #[async_trait::async_trait]
  impl InstalledTilesetsRepository for FailingTilesetRepository {
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
      Ok(())
    }

    async fn is_tileset_installed(
      &self,
      _tileset_id: &str,
      _game_variant: &GameVariant,
    ) -> Result<bool, InstalledTilesetsRepositoryError> {
      Err(InstalledTilesetsRepositoryError::IsInstalled(Box::new(
        std::io::Error::new(
          std::io::ErrorKind::Other,
          "database connection failed",
        ),
      )))
    }
  }

  #[tokio::test]
  async fn test_get_installation_status_repository_error()
  -> TestResult {
    let repo = FailingTilesetRepository;
    let variant = GameVariant::DarkDaysAhead;

    let result = get_third_party_tileset_installation_status(
      "error-tileset",
      &variant,
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(GetThirdPartyTilesetInstallationStatusError::Repository(
        InstalledTilesetsRepositoryError::IsInstalled(_)
      ))
    ));

    Ok(())
  }
}
