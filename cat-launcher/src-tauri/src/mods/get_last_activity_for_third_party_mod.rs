use reqwest::Client;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

use crate::infra::github::get_last_commit::{
  get_last_commit, GetLastCommitError,
};
use crate::mods::get_third_party_mod_by_id::{
  get_third_party_mod_by_id, GetThirdPartyModByIdError,
};
use crate::mods::repository::cached_mods_repository::CachedModsRepository;
use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LastModActivity {
  pub timestamp: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum GetLastActivityError {
  #[error("failed to get last commit: {0}")]
  GetLastCommit(#[from] GetLastCommitError),

  #[error("failed to get mod: {0}")]
  GetMod(#[from] GetThirdPartyModByIdError),

  #[error("mod has no github activity configured")]
  NoGithubActivity,
}

fn extract_repo(url_str: &str) -> Option<String> {
  let parsed_url = Url::parse(url_str).ok()?;

  let path = parsed_url.path();
  let parts: Vec<&str> = path
    .trim_start_matches('/')
    .trim_end_matches('/')
    .trim_end_matches(".git")
    .split('/')
    .collect();

  if parts.len() >= 2 {
    let owner = parts[0];
    let repo = parts[1];

    if !owner.is_empty() && !repo.is_empty() {
      Some(format!("{}/{}", owner, repo))
    } else {
      None
    }
  } else {
    None
  }
}

pub async fn get_last_activity_for_third_party_mod(
  mod_id: &str,
  variant: &GameVariant,
  client: &Client,
  cached_mods_repository: &dyn CachedModsRepository,
) -> Result<LastModActivity, GetLastActivityError> {
  let mod_data = get_third_party_mod_by_id(
    mod_id,
    variant,
    cached_mods_repository,
  )
  .await?;

  let github_url = mod_data.activity.github;

  let repo = extract_repo(&github_url)
    .ok_or(GetLastActivityError::NoGithubActivity)?;

  let last_commit = get_last_commit(&repo, client).await?;
  let last_commit_date = last_commit.commit.author.date;
  let timestamp = last_commit_date.timestamp_millis();

  Ok(LastModActivity { timestamp })
}
