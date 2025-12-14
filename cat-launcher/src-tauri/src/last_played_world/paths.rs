use crate::filesystem::paths::get_or_create_user_game_data_dir;
use crate::filesystem::paths::GetUserGameDataDirError;
use crate::variants::GameVariant;
use std::path::Path;
use std::path::PathBuf;

pub async fn get_last_world_path(
  data_dir: &Path,
  variant: &GameVariant,
) -> Result<PathBuf, GetUserGameDataDirError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  Ok(user_data_dir.join("config").join("lastworld.json"))
}
