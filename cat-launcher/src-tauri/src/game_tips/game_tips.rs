use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::paths::{get_tip_file_paths, GetTipFilePathsError};
use crate::infra::utils::OS;
use crate::last_played::last_played::LastPlayedError;
use crate::repository::last_played_repository::LastPlayedVersionRepository;
use crate::variants::GameVariant;

#[derive(Debug, Deserialize, Clone)]
pub struct Tip {
    pub text: Vec<String>,
}

#[derive(Debug, Error)]
pub enum GetAllTipsForVariantError {
    #[error("Failed to get last played version: {0}")]
    LastPlayed(#[from] LastPlayedError),

    #[error("Failed to get tip file paths: {0}")]
    GetTipFilePaths(#[from] GetTipFilePathsError),

    #[error("Tokio IO error: {0}")]
    Tokio(#[from] tokio::io::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub async fn get_all_tips_for_variant(
    variant: &GameVariant,
    data_dir: &Path,
    os: &OS,
    last_played_repository: &dyn LastPlayedVersionRepository,
) -> Result<Vec<String>, GetAllTipsForVariantError> {
    let last_played_version = match variant
        .get_last_played_version(last_played_repository)
        .await?
    {
        Some(version) => version,
        None => return Ok(vec![]),
    };

    let tip_file_paths = get_tip_file_paths(variant, &last_played_version, data_dir, os).await?;
    let mut all_tips: Vec<String> = Vec::new();

    for path in tip_file_paths {
        if path.exists() {
            let tips_file_content = tokio::fs::read_to_string(path).await?;
            if !tips_file_content.is_empty() {
                let tips: Vec<Tip> = serde_json::from_str(&tips_file_content)?;
                all_tips.extend(tips.into_iter().flat_map(|tip| tip.text));
            }
        }
    }

    Ok(all_tips)
}
