use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::infra::archive::{
  create_zip_archive, ArchiveCreationError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ExportGameDataError {
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to create archive: {0}")]
  ArchiveCreation(#[from] ArchiveCreationError),
}

pub async fn export_game_data(
  variant: &GameVariant,
  data_dir: &Path,
  destination_path: PathBuf,
) -> Result<(), ExportGameDataError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;

  let paths_to_include = vec![user_data_dir.clone()];

  create_zip_archive(
    &user_data_dir,
    &paths_to_include,
    &destination_path,
  )
  .await?;

  Ok(())
}
