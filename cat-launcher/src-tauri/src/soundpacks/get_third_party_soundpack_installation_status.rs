use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::soundpacks::types::SoundpackInstallationStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartySoundpackInstallationStatusError {
  #[error("failed to check soundpack installation status: {0}")]
  Repository(#[from] InstalledSoundpacksRepositoryError),
}

pub async fn get_third_party_soundpack_installation_status(
  soundpack_id: &str,
  variant: &GameVariant,
  repository: &dyn InstalledSoundpacksRepository,
) -> Result<
  SoundpackInstallationStatus,
  GetThirdPartySoundpackInstallationStatusError,
> {
  let is_installed = repository
    .is_soundpack_installed(soundpack_id, variant)
    .await?;

  Ok(if is_installed {
    SoundpackInstallationStatus::Installed
  } else {
    SoundpackInstallationStatus::NotInstalled
  })
}
