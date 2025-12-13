use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::types::ModInstallationStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModInstallationStatusError {
  #[error("failed to check mod installation status: {0}")]
  Repository(#[from] InstalledModsRepositoryError),
}

pub async fn get_third_party_mod_installation_status(
  mod_id: &str,
  variant: &GameVariant,
  repository: &dyn InstalledModsRepository,
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
