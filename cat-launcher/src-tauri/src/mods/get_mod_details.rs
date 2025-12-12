use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::mods::mod_info::{ModsJson, ThirdPartyMod};
use crate::mods::validation::{validate_mod_id, InvalidModIdError};
use crate::variants::GameVariant;

#[derive(Error, Debug)]
pub enum GetModDetailsError {
    #[error("invalid mod ID: {0}")]
    InvalidModId(#[from] InvalidModIdError),

    #[error("failed to read mods.json: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse mods.json: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("mod with ID {0} not found")]
    NotFound(String),
}

pub async fn get_mod_details(
    variant: &GameVariant,
    mod_id: &str,
    resource_dir: &Path,
) -> Result<ThirdPartyMod, GetModDetailsError> {
    // Validate mod_id to prevent path traversal and ensure safety
    validate_mod_id(mod_id)?;

    let mods_json_path = resource_dir.join("mods.json");
    let mods_json_content = fs::read_to_string(mods_json_path).await?;

    let mods_data: ModsJson = serde_json::from_str(&mods_json_content)?;

    mods_data
        .get(variant.id())
        .and_then(|mods| mods.iter().find(|m| m.id == mod_id))
        .map(|mod_entry| mod_entry.clone().into())
        .ok_or_else(|| GetModDetailsError::NotFound(mod_id.to_string()))
}
