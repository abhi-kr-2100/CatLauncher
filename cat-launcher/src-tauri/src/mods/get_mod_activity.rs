use std::path::Path;
use reqwest::Client;

use crate::infra::github::api::{get_latest_commit_timestamp, GetLatestCommitTimestampError};
use crate::mods::get_third_party_mod::{get_third_party_mod, GetThirdPartyModError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetModActivityError {
    #[error("failed to get third party mod: {0}")]
    GetThirdPartyMod(#[from] GetThirdPartyModError),

    #[error("mod with id `{0}` has no github activity configured")]
    NoGithubActivity(String),

    #[error("failed to get latest commit timestamp: {0}")]
    GetLatestCommitTimestamp(#[from] GetLatestCommitTimestampError),
}

pub async fn get_mod_activity(
    mod_id: &str,
    game_variant: &GameVariant,
    resource_dir: &Path,
    http_client: &Client,
) -> Result<String, GetModActivityError> {
    let third_party_mod = get_third_party_mod(mod_id, game_variant, resource_dir).await?;

    if third_party_mod.activity.activity_type == "github_commit" {
        if let Some(github_url) = third_party_mod.activity.github {
            let timestamp = get_latest_commit_timestamp(&github_url, http_client).await?;
            return Ok(timestamp);
        }
    }

    Err(GetModActivityError::NoGithubActivity(mod_id.to_string()))
}
