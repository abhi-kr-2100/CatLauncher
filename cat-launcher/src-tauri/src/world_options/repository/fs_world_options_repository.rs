use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs;

use crate::filesystem::paths::get_or_create_user_game_data_dir;
use crate::variants::GameVariant;
use crate::world_options::repository::{
  WorldOptionsError, WorldOptionsRepository,
};
use crate::world_options::types::{World, WorldOption};

pub struct FsWorldOptionsRepository {
  data_dir: PathBuf,
}

impl FsWorldOptionsRepository {
  pub fn new(data_dir: PathBuf) -> Self {
    Self { data_dir }
  }
}

async fn collect_worlds_from_dir(
  save_dir: &Path,
) -> Result<Vec<World>, WorldOptionsError> {
  let mut entries = fs::read_dir(save_dir).await?;
  let mut worlds = Vec::new();

  while let Some(entry) = entries.next_entry().await? {
    let path = entry.path();
    if entry.file_type().await?.is_dir() {
      let options_path = path.join("worldoptions.json");
      if fs::try_exists(&options_path).await.unwrap_or(false) {
        if let Ok(name) = entry.file_name().into_string() {
          worlds.push(World { name });
        }
      }
    }
  }

  Ok(worlds)
}

#[async_trait]
impl WorldOptionsRepository for FsWorldOptionsRepository {
  async fn get_worlds(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<World>, WorldOptionsError> {
    let user_data_dir =
      get_or_create_user_game_data_dir(variant, &self.data_dir)
        .await?;
    let save_dir = user_data_dir.join("save");

    if fs::try_exists(&save_dir).await? {
      collect_worlds_from_dir(&save_dir).await
    } else {
      Ok(vec![])
    }
  }

  async fn get_world_options(
    &self,
    variant: &GameVariant,
    world: &str,
  ) -> Result<Vec<WorldOption>, WorldOptionsError> {
    let user_data_dir =
      get_or_create_user_game_data_dir(variant, &self.data_dir)
        .await?;
    let options_path = user_data_dir
      .join("save")
      .join(world)
      .join("worldoptions.json");

    if fs::try_exists(&options_path).await? {
      let content = fs::read_to_string(options_path).await?;
      let options: Vec<WorldOption> = serde_json::from_str(&content)?;
      Ok(options)
    } else {
      Ok(vec![])
    }
  }

  async fn update_world_options(
    &self,
    variant: &GameVariant,
    world: &str,
    options: &[WorldOption],
  ) -> Result<(), WorldOptionsError> {
    let user_data_dir =
      get_or_create_user_game_data_dir(variant, &self.data_dir)
        .await?;
    let options_path = user_data_dir
      .join("save")
      .join(world)
      .join("worldoptions.json");

    let tmp_path = options_path.with_extension("json.tmp");

    let content = serde_json::to_string_pretty(&options)?;

    {
      use tokio::io::AsyncWriteExt;
      let mut file = fs::File::create(&tmp_path).await?;
      file.write_all(content.as_bytes()).await?;
      file.sync_all().await?;
    }

    if fs::try_exists(&options_path).await? {
      fs::remove_file(&options_path).await?;
    }
    fs::rename(tmp_path, options_path).await?;

    Ok(())
  }
}
