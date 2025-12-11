use async_trait::async_trait;

use crate::mods::models::ThirdPartyModStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledModsRepositoryError {
    #[error("failed to list installed mods: {0}")]
    List(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to get installed mod: {0}")]
    Get(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to upsert installed mod: {0}")]
    Upsert(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to delete installed mod: {0}")]
    Delete(Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait InstalledModsRepository: Send + Sync {
    async fn list_installed_mods(
        &self,
        variant: &GameVariant,
    ) -> Result<Vec<ThirdPartyModStatus>, InstalledModsRepositoryError>;

    async fn get_installed_mod(
        &self,
        variant: &GameVariant,
        mod_id: &str,
    ) -> Result<Option<ThirdPartyModStatus>, InstalledModsRepositoryError>;

    async fn upsert_installed_mod(
        &self,
        status: &ThirdPartyModStatus,
    ) -> Result<(), InstalledModsRepositoryError>;

    async fn delete_installed_mod(
        &self,
        variant: &GameVariant,
        mod_id: &str,
    ) -> Result<(), InstalledModsRepositoryError>;
}
