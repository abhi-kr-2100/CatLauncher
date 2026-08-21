use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::tilesets::types::TilesetInstallationStatus;
use crate::variants::GameVariant;

/// Errors that can occur when retrieving the installation status of a third-party tileset.
#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyTilesetInstallationStatusError {
  /// An error occurred in the underlying repository.
  #[error("failed to check tileset installation status: {0}")]
  Repository(#[from] InstalledTilesetsRepositoryError),
}

/// Checks if a third-party tileset is currently installed for a given game variant.
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
