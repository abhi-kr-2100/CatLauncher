use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::types::ModInstallationStatus;
use crate::variants::GameVariant;

/// Errors that can occur when checking the installation status of a third-party mod.
#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModInstallationStatusError {
  /// Failed to check mod installation status via the repository.
  #[error("failed to check mod installation status: {0}")]
  Repository(#[from] InstalledModsRepositoryError),
}

/// Checks the installation status of a third-party mod for a specific game variant.
pub async fn get_third_party_mod_installation_status(
  mod_id: &str,
  variant: &GameVariant,
  repository: &impl InstalledModsRepository,
) -> Result<
  ModInstallationStatus,
  GetThirdPartyModInstallationStatusError,
> {
  let is_installed =
    repository.is_mod_installed(mod_id, variant).await?;

  Ok(if is_installed {
    ModInstallationStatus::Installed
  } else {
    ModInstallationStatus::NotInstalled
  })
}
