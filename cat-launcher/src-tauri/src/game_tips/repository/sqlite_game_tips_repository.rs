use async_trait::async_trait;
use tauri::{AppHandle, Manager};

use crate::{
  active_release::repository::ActiveReleaseRepository,
  fetch_releases::repository::ReleasesRepository,
  game_tips::lib::{self, GetAllTipsForVariantError},
  infra::utils::{get_os_enum, OS},
  variants::GameVariant,
};

use super::game_tips_repository::{
  GameTipsRepository, GameTipsRepositoryError,
};

pub struct SqliteGameTipsRepository<'a> {
  app_handle: AppHandle,
  os: OS,
  active_release_repository:
    &'a (dyn ActiveReleaseRepository + Send + Sync),
  releases_repository: &'a (dyn ReleasesRepository + Send + Sync),
}

impl<'a> SqliteGameTipsRepository<'a> {
  pub fn new(
    app_handle: AppHandle,
    active_release_repository: &'a (dyn ActiveReleaseRepository
           + Send
           + Sync),
    releases_repository: &'a (dyn ReleasesRepository + Send + Sync),
  ) -> Result<Self, GameTipsRepositoryError> {
    let os = get_os_enum(std::env::consts::OS)
      .map_err(GetAllTipsForVariantError::from)
      .map_err(GameTipsRepositoryError::from)?;
    Ok(Self {
      app_handle,
      os,
      active_release_repository,
      releases_repository,
    })
  }
}

#[async_trait]
impl GameTipsRepository for SqliteGameTipsRepository<'_> {
  async fn get_all_tips_for_variant(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<String>, GameTipsRepositoryError> {
    let data_dir = self
      .app_handle
      .path()
      .app_local_data_dir()
      .map_err(GetAllTipsForVariantError::from)
      .map_err(GameTipsRepositoryError::from)?;
    let tips = lib::get_all_tips_for_variant(
      variant,
      &data_dir,
      &self.os,
      self.active_release_repository,
      self.releases_repository,
    )
    .await
    .map_err(GameTipsRepositoryError::from)?;
    Ok(tips)
  }
}
