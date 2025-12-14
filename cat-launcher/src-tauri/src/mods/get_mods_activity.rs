use futures::future::join_all;
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;

use crate::infra::github::api::{
  get_latest_commit, GetLatestCommitError,
};
use crate::mods::get_third_party_mod::{
  get_third_party_mod, GetThirdPartyModError,
};
use crate::variants::GameVariant;

type ModsActivityFutureResult =
  Result<(String, Option<String>), GetModsActivityError>;

#[derive(thiserror::Error, Debug)]
pub enum GetModsActivityError {
  #[error("failed to get third party mod: {0}")]
  GetThirdPartyMod(#[from] GetThirdPartyModError),

  #[error("failed to get latest commit: {0}")]
  GetLatestCommit(#[from] GetLatestCommitError),
}

pub async fn get_mods_activity(
  mod_ids: Vec<String>,
  game_variant: &GameVariant,
  resource_dir: &Path,
  http_client: &Client,
) -> Result<HashMap<String, Option<String>>, GetModsActivityError> {
  let mut futures = Vec::new();

  for mod_id in mod_ids {
    let future = async {
      let third_party_mod =
        get_third_party_mod(&mod_id, game_variant, resource_dir)
          .await?;

      if third_party_mod.activity.activity_type == "github_commit" {
        if let Some(github_url) = third_party_mod.activity.github {
          let commit =
            get_latest_commit(&github_url, http_client).await?;
          let timestamp = commit.map(|c| c.commit.committer.date);
          return Ok((mod_id, timestamp));
        }
      }

      Ok((mod_id, None))
    };
    futures.push(future);
  }

  let results: Vec<ModsActivityFutureResult> =
    join_all(futures).await;
  let mut activity_map = HashMap::new();

  for result in results {
    let (mod_id, timestamp) = result?;
    activity_map.insert(mod_id, timestamp);
  }

  Ok(activity_map)
}
