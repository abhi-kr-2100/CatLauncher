use std::collections::HashMap;
use std::io;
use std::path::Path;
use serde::Deserialize;
use tokio::fs::read_to_string;
use reqwest::Client;
use crate::mods::paths::get_mods_resource_path;
use crate::mods::types::{ThirdPartyMod};
use crate::variants::GameVariant;


#[derive(Debug, Deserialize)]
struct GitHubCommit {
    commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    committer: Signature,
}

#[derive(Debug, Deserialize)]
struct Signature {
    date: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetModActivityError {
    #[error("failed to read mods.json: {0}")]
    ReadModsJson(#[from] io::Error),

    #[error("failed to parse mods.json: {0}")]
    ParseModsJson(#[from] serde_json::Error),

    #[error("mod with id `{0}` not found")]
    ModNotFound(String),

    #[error("mod with id `{0}` has no github activity configured")]
    NoGithubActivity(String),

    #[error("failed to make request to github api: {0}")]
    GitHubApiRequest(#[from] reqwest::Error),

    #[error("failed to parse github url `{0}`")]
    ParseGithubUrl(String),
}

pub async fn get_mod_activity(
    mod_id: &str,
    game_variant: &GameVariant,
    resource_dir: &Path,
    http_client: &Client,
) -> Result<String, GetModActivityError> {
    // Construct the path to mods.json
    let mods_json_path = get_mods_resource_path(resource_dir);

    // Try to read the mods.json file
    let content = match read_to_string(&mods_json_path).await {
        Ok(content) => content,
        Err(e) => return Err(GetModActivityError::ReadModsJson(e)),
    };

    let mods_data: HashMap<
        GameVariant,
        HashMap<String, serde_json::Value>,
    > = serde_json::from_str(&content)?;

    let variant_mods = match mods_data.get(game_variant) {
        Some(mods) => mods,
        None => return Err(GetModActivityError::ModNotFound(mod_id.to_string())),
    };

    let mod_data = match variant_mods.get(mod_id) {
        Some(mod_data) => mod_data,
        None => return Err(GetModActivityError::ModNotFound(mod_id.to_string())),
    };

    let third_party_mod: ThirdPartyMod = serde_json::from_value(mod_data.clone())?;

    if third_party_mod.activity.activity_type == "github_commit" {
        if let Some(github_url) = third_party_mod.activity.github {
            let url_parts: Vec<&str> = github_url.split('/').collect();
            let owner = url_parts.get(3).ok_or_else(|| GetModActivityError::ParseGithubUrl(github_url.clone()))?;
            let repo = url_parts.get(4).ok_or_else(|| GetModActivityError::ParseGithubUrl(github_url.clone()))?;


            let url = format!("https://api.github.com/repos/{}/{}/commits", owner, repo);

            let response = http_client.get(&url).send().await?;
            let commits: Vec<GitHubCommit> = response.json().await?;

            if let Some(latest_commit) = commits.first() {
                return Ok(latest_commit.commit.committer.date.clone());
            }
        }
    }

    Err(GetModActivityError::NoGithubActivity(mod_id.to_string()))
}
