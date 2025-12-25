use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

use reqwest::Client;
use serde::Serialize;
use ts_rs::TS;

use crate::active_release::repository::ActiveReleaseRepository;
use crate::infra::utils::{Asset, OS};
use crate::mods::bright_nights::fetch_bright_nights_mods;
use crate::mods::bright_nights::FetchBrightNightsModsError;
use crate::mods::list_all_local_mods::{
  list_all_local_mods, ListAllLocalModsError,
};
use crate::mods::list_third_party_mods_from_resource::{
  list_third_party_mods_from_resource,
  ListThirdPartyModsFromResourceError,
};
use crate::mods::repository::cached_mods_repository::{
  CachedModsRepository, CachedModsRepositoryError,
};
use crate::mods::types::{Mod, ThirdPartyMod};
use crate::variants::GameVariant;

fn sort_mods_by_priority(mods: &mut [Mod]) {
  mods.sort_by(|a, b| {
    let type_priority = |m: &Mod| match m {
      Mod::Stock(_) => 0,
      Mod::ThirdParty(_) => 1,
    };

    type_priority(a)
      .cmp(&type_priority(b))
      .then_with(|| a.id().cmp(b.id()))
  });
}

fn dedupe_third_party_mods(
  priority_ordered_lists: &[Vec<ThirdPartyMod>],
) -> Vec<ThirdPartyMod> {
  let mut seen_ids = HashSet::<String>::new();
  let mut result = Vec::new();

  for list in priority_ordered_lists {
    for third_party_mod in list {
      if seen_ids.insert(third_party_mod.id.clone()) {
        result.push(third_party_mod.clone());
      }
    }
  }

  result
}

fn combine_stock_and_third_party(
  stock_mods: &[Mod],
  third_party_mods: &[ThirdPartyMod],
) -> Vec<Mod> {
  let stock_ids: HashSet<&str> =
    stock_mods.iter().map(|m| m.id()).collect();

  let mut mods = stock_mods.to_vec();

  for third_party_mod in third_party_mods {
    if stock_ids.contains(third_party_mod.id.as_str()) {
      continue;
    }

    mods.push(Mod::ThirdParty(third_party_mod.clone()));
  }

  sort_mods_by_priority(&mut mods);

  mods
}

#[derive(thiserror::Error, Debug)]
pub enum FetchModsError<E: Error> {
  #[error("failed to list stock mods: {0}")]
  ListStockMods(#[from] ListAllLocalModsError),

  #[error("failed to read cached mods: {0}")]
  CachedMods(#[from] CachedModsRepositoryError),

  #[error("failed to list mods from mods.json resource file: {0}")]
  ResourceMods(#[from] ListThirdPartyModsFromResourceError),

  #[error("failed to fetch online mods: {0}")]
  OnlineMods(#[from] FetchBrightNightsModsError),

  #[error("failed to send mods update: {0}")]
  Send(E),
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ModsUpdatePayload {
  pub variant: GameVariant,
  pub mods: Vec<Mod>,
  pub status: ModsUpdateStatus,
}

#[derive(Debug, Clone, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ModsUpdateStatus {
  Fetching,
  Success,
  Error,
}

impl GameVariant {
  async fn fetch_online_mods_for_variant(
    &self,
    client: &Client,
  ) -> Result<Vec<ThirdPartyMod>, FetchBrightNightsModsError> {
    match self {
      GameVariant::BrightNights => {
        fetch_bright_nights_mods(client).await
      }
      GameVariant::DarkDaysAhead | GameVariant::TheLastGeneration => {
        Ok(Vec::new())
      }
    }
  }

  #[allow(clippy::too_many_arguments)]
  pub async fn fetch_mods<E, F>(
    &self,
    client: &Client,
    data_dir: &Path,
    resources_dir: &Path,
    os: &OS,
    active_release_repository: &dyn ActiveReleaseRepository,
    cached_mods_repository: &dyn CachedModsRepository,
    on_mods: F,
  ) -> Result<(), FetchModsError<E>>
  where
    E: Error,
    F: Fn(ModsUpdatePayload) -> Result<(), E>,
  {
    let stock_mods = list_all_local_mods(
      self,
      data_dir,
      os,
      active_release_repository,
    )
    .await?;

    let cached_mods =
      cached_mods_repository.get_cached_mods(self).await?;

    let resource_mods =
      list_third_party_mods_from_resource(self, resources_dir)
        .await?;

    let cached_and_resource = dedupe_third_party_mods(&[
      cached_mods.clone(),
      resource_mods.clone(),
    ]);

    cached_mods_repository
      .update_cached_mods(self, &cached_and_resource)
      .await?;

    let initial_mods = combine_stock_and_third_party(
      &stock_mods,
      &cached_and_resource,
    );

    on_mods(ModsUpdatePayload {
      variant: *self,
      mods: initial_mods,
      status: ModsUpdateStatus::Fetching,
    })
    .map_err(FetchModsError::Send)?;

    let online_mods =
      self.fetch_online_mods_for_variant(client).await?;

    let final_third_party = dedupe_third_party_mods(&[
      online_mods,
      cached_and_resource,
      resource_mods,
    ]);

    cached_mods_repository
      .update_cached_mods(self, &final_third_party)
      .await?;

    let final_mods =
      combine_stock_and_third_party(&stock_mods, &final_third_party);

    on_mods(ModsUpdatePayload {
      variant: *self,
      mods: final_mods,
      status: ModsUpdateStatus::Success,
    })
    .map_err(FetchModsError::Send)?;

    Ok(())
  }
}
