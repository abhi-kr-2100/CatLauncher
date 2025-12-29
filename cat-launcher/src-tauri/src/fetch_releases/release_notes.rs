use std::collections::HashSet;
use std::error::Error;

use reqwest::Client;
use serde::Serialize;
use ts_rs::TS;

use crate::fetch_releases::repository::{
  ReleaseNotesRepository, ReleasesRepository,
};
use crate::infra::github::utils::fetch_github_release_by_tag;
use crate::infra::utils::get_github_repo_for_variant;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchReleaseNotesError<E: Error> {
  #[error("failed to send release notes update: {0}")]
  Send(E),
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReleaseNotesUpdatePayload {
  pub request_id: String,
  pub variant: GameVariant,
  pub version: Option<String>,
  pub notes: Option<String>,
  pub status: ReleaseNotesUpdateStatus,
}

#[derive(Debug, Clone, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ReleaseNotesUpdateStatus {
  Fetching,
  Cached,
  Fetched,
  Error,
  Complete,
}

impl GameVariant {
  pub async fn fetch_release_notes<E, F>(
    &self,
    request_id: &str,
    client: &Client,
    release_notes_repository: &dyn ReleaseNotesRepository,
    releases_repository: &dyn ReleasesRepository,
    versions: &[String],
    on_update: F,
  ) -> Result<(), FetchReleaseNotesError<E>>
  where
    E: Error,
    F: Fn(ReleaseNotesUpdatePayload) -> Result<(), E>,
  {
    let request_id = request_id.to_string();

    on_update(ReleaseNotesUpdatePayload {
      request_id: request_id.clone(),
      variant: *self,
      version: None,
      notes: None,
      status: ReleaseNotesUpdateStatus::Fetching,
    })
    .map_err(FetchReleaseNotesError::Send)?;

    let cached_notes = match release_notes_repository
      .get_release_notes_by_tag_names(self, versions)
      .await
    {
      Ok(notes) => notes,
      Err(e) => {
        eprintln!("Failed to read cached release notes: {e}");
        on_update(ReleaseNotesUpdatePayload {
          request_id: request_id.clone(),
          variant: *self,
          version: None,
          notes: None,
          status: ReleaseNotesUpdateStatus::Error,
        })
        .map_err(FetchReleaseNotesError::Send)?;
        Default::default()
      }
    };

    let mut already_emitted = HashSet::new();
    let mut missing = Vec::new();

    for version in versions {
      if !already_emitted.insert(version.clone()) {
        continue;
      }

      let maybe_notes = cached_notes.get(version).cloned().flatten();

      if let Some(notes) = maybe_notes {
        on_update(ReleaseNotesUpdatePayload {
          request_id: request_id.clone(),
          variant: *self,
          version: Some(version.clone()),
          notes: Some(notes),
          status: ReleaseNotesUpdateStatus::Cached,
        })
        .map_err(FetchReleaseNotesError::Send)?;
      } else {
        missing.push(version.clone());
      }
    }

    let repo = get_github_repo_for_variant(self);

    for version in missing {
      match fetch_github_release_by_tag(client, repo, &version).await
      {
        Ok(release) => {
          let notes = release.body.clone();

          if let Err(e) = releases_repository
            .update_cached_releases(self, &[release])
            .await
          {
            eprintln!("Failed to save release notes to cache: {e}");
          }

          on_update(ReleaseNotesUpdatePayload {
            request_id: request_id.clone(),
            variant: *self,
            version: Some(version),
            notes,
            status: ReleaseNotesUpdateStatus::Fetched,
          })
          .map_err(FetchReleaseNotesError::Send)?;
        }
        Err(e) => {
          eprintln!("Failed to fetch release notes from GitHub: {e}");
          on_update(ReleaseNotesUpdatePayload {
            request_id: request_id.clone(),
            variant: *self,
            version: Some(version),
            notes: None,
            status: ReleaseNotesUpdateStatus::Error,
          })
          .map_err(FetchReleaseNotesError::Send)?;
        }
      }
    }

    on_update(ReleaseNotesUpdatePayload {
      request_id,
      variant: *self,
      version: None,
      notes: None,
      status: ReleaseNotesUpdateStatus::Complete,
    })
    .map_err(FetchReleaseNotesError::Send)?;

    Ok(())
  }
}
