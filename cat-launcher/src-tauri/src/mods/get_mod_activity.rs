use std::path::Path;
use reqwest::Client;

use crate::infra::github::api::{get_latest_commit, GetLatestCommitError};
use crate::mods::get_third_party_mod::{get_third_party_mod, GetThirdPartyModError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetModActivityError {
    #[error("failed to get third party mod: {0}")]
    GetThirdPartyMod(#[from] GetThirdPartyModError),

    #[error("mod with id `{0}` has no github activity configured")]
    NoGithubActivity(String),

    #[error("failed to get latest commit: {0}")]
    GetLatestCommit(#[from] GetLatestCommitError),

    #[error("failed to parse github url `{0}`")]
    ParseGithubUrl(String),
}

pub async fn get_mod_activity(
    mod_id: &str,
    game_variant: &GameVariant,
    resource_dir: &Path,
    http_client: &Client,
) -> Result<Option<String>, GetModActivityError> {
    let third_party_mod = get_third_party_mod(mod_id, game_variant, resource_dir).await?;

    if third_party_mod.activity.activity_type == "github_commit" {
        if let Some(github_url) = third_party_mod.activity.github {
            let url_parts: Vec<&str> = github_url.split('/').collect();
            let owner = url_parts.get(3).ok_or_else(|| GetModActivityError::ParseGithubUrl(github_url.clone()))?;
            let repo = url_parts.get(4).ok_or_else(|| GetModActivityError::ParseGithubUrl(github_url.clone()))?;

            let commit = get_latest_commit(owner, repo, http_client).await?;
            let timestamp = commit.map(|c| c.commit.committer.date);
            return Ok(timestamp);
        }
    }

    Err(GetModActivityError::NoGithubActivity(mod_id.to_string()))
}
