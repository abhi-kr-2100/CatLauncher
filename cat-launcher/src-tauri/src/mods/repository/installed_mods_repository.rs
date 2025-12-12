use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledModsRepositoryError {
    #[error("failed to add installed mod: {0}")]
    Add(Box<dyn Error + Send + Sync>),

    #[error("failed to get installed mods: {0}")]
    Get(Box<dyn Error + Send + Sync>),

    #[error("failed to delete installed mod: {0}")]
    Delete(Box<dyn Error + Send + Sync>),

    #[error("failed to check if mod is installed: {0}")]
    Check(Box<dyn Error + Send + Sync>),
}

/// Repository trait for managing installed mods.
///
/// Provides CRUD operations for tracking which mods are currently installed.
/// All operations are asynchronous and return specific error variants on failure.
#[async_trait]
pub trait InstalledModsRepository: Send + Sync {
    /// Adds a mod to the installed mods list for a specific game variant.
    ///
    /// # Arguments
    /// * `mod_id` - The unique identifier of the mod to add
    /// * `game_variant` - The game variant this mod is installed for
    ///
    /// # Errors
    /// Returns InstalledModsRepositoryError::Add if the operation fails.
    async fn add_installed_mod(&self, mod_id: &str, game_variant: &GameVariant) -> Result<(), InstalledModsRepositoryError>;

    /// Retrieves all installed mod IDs for a specific game variant.
    ///
    /// # Arguments
    /// * `game_variant` - The game variant to get installed mods for
    ///
    /// # Errors
    /// Returns InstalledModsRepositoryError::Get if the operation fails.
    async fn get_all_installed_mods(&self, game_variant: &GameVariant) -> Result<Vec<String>, InstalledModsRepositoryError>;

    /// Checks if a specific mod is installed for a specific game variant.
    ///
    /// # Arguments
    /// * `mod_id` - The unique identifier of the mod to check
    /// * `game_variant` - The game variant to check for
    ///
    /// # Errors
    /// Returns InstalledModsRepositoryError::Check if the operation fails.
    async fn is_mod_installed(&self, mod_id: &str, game_variant: &GameVariant) -> Result<bool, InstalledModsRepositoryError>;

    /// Removes a mod from the installed mods list for a specific game variant.
    ///
    /// # Arguments
    /// * `mod_id` - The unique identifier of the mod to delete
    /// * `game_variant` - The game variant to remove the mod from
    ///
    /// # Errors
    /// Returns InstalledModsRepositoryError::Delete if the operation fails.
    async fn delete_installed_mod(&self, mod_id: &str, game_variant: &GameVariant) -> Result<(), InstalledModsRepositoryError>;
}
