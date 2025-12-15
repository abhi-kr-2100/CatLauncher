use reqwest::Client;
use std::path::Path;

use crate::infra::github::api::{
  get_latest_commit, GetLatestCommitError,
};
use crate::mods::get_third_party_mod::{
  get_third_party_mod, GetThirdPartyModError,
};
use crate::mods::types::ActivityType;
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
  let third_party_mod =
    get_third_party_mod(mod_id, game_variant, resource_dir).await?;

  if third_party_mod.activity.activity_type
    == ActivityType::GithubCommit
  {
    if let Some(github_url) = third_party_mod.activity.github {
      let url = url::Url::parse(&github_url).map_err(|_| {
        GetModActivityError::ParseGithubUrl(github_url.clone())
      })?;
      let path_segments = url.path_segments().ok_or_else(|| {
        GetModActivityError::ParseGithubUrl(github_url.clone())
      })?;
      let mut path_parts = path_segments.rev();
      let repo = path_parts.next().ok_or_else(|| {
        GetModActivityError::ParseGithubUrl(github_url.clone())
      })?;
      let owner = path_parts.next().ok_or_else(|| {
        GetModActivityError::ParseGithubUrl(github_url.clone())
      })?;

      let commit =
        get_latest_commit(owner, repo, http_client).await?;
      let timestamp = commit.map(|c| c.commit.committer.date);
      return Ok(timestamp);
    }
  }

  Err(GetModActivityError::NoGithubActivity(mod_id.to_string()))
}
