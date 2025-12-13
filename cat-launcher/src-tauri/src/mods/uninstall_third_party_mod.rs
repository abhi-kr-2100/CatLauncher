use crate::mods::repository::installed_mods_repository::{
    InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyModError {
    #[error("failed to remove installed mod from repository: {0}")]
    Repository(#[from] InstalledModsRepositoryError),
}

pub async fn uninstall_third_party_mod(
    mod_id: &str,
    game_variant: &GameVariant,
    repository: &impl InstalledModsRepository,
) -> Result<(), UninstallThirdPartyModError> {
    repository
        .delete_installed_mod(mod_id, game_variant)
        .await?;
    Ok(())
}
