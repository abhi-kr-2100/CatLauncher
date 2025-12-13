use crate::mods::repository::installed_mods_repository::{
    InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyModError {
    #[error("failed to save installed mod to repository: {0}")]
    Repository(#[from] InstalledModsRepositoryError),
}

pub async fn install_third_party_mod(
    mod_id: &str,
    game_variant: &GameVariant,
    repository: &impl InstalledModsRepository,
) -> Result<(), InstallThirdPartyModError> {
    repository
        .add_installed_mod(mod_id, game_variant)
        .await?;
    Ok(())
}
