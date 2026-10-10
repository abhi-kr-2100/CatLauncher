use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetGameExecutableDirError, get_game_resources_dir,
};
use crate::infra::utils::OS;
use crate::mods::types::OnlineModRepository;
use crate::variants::GameVariant;

#[derive(Default)]
pub struct OnlineModRepositoryRegistry {
  repositories: Vec<Box<dyn OnlineModRepository>>,
}

impl OnlineModRepositoryRegistry {
  pub fn register(
    &mut self,
    repository: Box<dyn OnlineModRepository>,
  ) {
    self.repositories.push(repository);
  }

  pub fn repositories(&self) -> &[Box<dyn OnlineModRepository>] {
    &self.repositories
  }
}

#[derive(thiserror::Error, Debug)]
pub enum GetStockModsDirError {
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

pub async fn get_stock_mods_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetStockModsDirError> {
  let game_resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  Ok(game_resources_dir.join("data").join("mods"))
}

pub fn get_mods_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("mods.json")
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use async_trait::async_trait;
  use tempfile::tempdir;

  use crate::filesystem::paths::get_or_create_asset_installation_dir;
  use crate::infra::http_client::HttpClient;
  use crate::mods::types::{FetchOnlineModsError, ThirdPartyMod};

  struct DummyOnlineRepo;

  #[async_trait]
  impl OnlineModRepository for DummyOnlineRepo {
    async fn get_mods_for_variant(
      &self,
      _variant: &GameVariant,
      _client: &dyn HttpClient,
    ) -> Result<Vec<ThirdPartyMod>, FetchOnlineModsError> {
      Ok(Vec::new())
    }
  }

  #[test]
  fn test_online_mod_repository_registry() {
    let mut registry = OnlineModRepositoryRegistry::default();
    assert!(registry.repositories().is_empty());

    registry.register(Box::new(DummyOnlineRepo));
    assert_eq!(registry.repositories().len(), 1);
  }

  #[test]
  #[ignore = "known bug: OnlineModRepositoryRegistry does not deduplicate repeated registrations"]
  fn test_online_mod_repository_registry_deduplication() {
    let mut registry = OnlineModRepositoryRegistry::default();
    registry.register(Box::new(DummyOnlineRepo));
    registry.register(Box::new(DummyOnlineRepo));
    assert_eq!(registry.repositories().len(), 1);
  }

  #[test]
  fn test_get_mods_resource_path() {
    let base = Path::new("custom").join("path");
    let path = get_mods_resource_path(&base);
    assert_eq!(path, base.join("content").join("mods.json"));
  }

  #[tokio::test]
  async fn test_get_stock_mods_dir()
  -> Result<(), Box<dyn std::error::Error>> {
    let temp_data = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let version = "0.H";
    let os = OS::Linux;

    // Create the Linux installation dir and cataclysm executable dir
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda-0.H");
    tokio::fs::create_dir_all(&game_dir).await?;

    let stock_mods_dir =
      get_stock_mods_dir(&variant, version, temp_data.path(), &os)
        .await?;

    assert_eq!(stock_mods_dir, game_dir.join("data").join("mods"));

    Ok(())
  }
}
