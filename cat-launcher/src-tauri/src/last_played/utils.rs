use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use crate::infra::utils::get_safe_filename;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum LastPlayedFileError {
    #[error("failed to create directory: {0}")]
    CreateDir(#[from] std::io::Error),
}

pub fn get_last_played_file_path(
    variant: &GameVariant,
    data_dir: &Path,
) -> Result<PathBuf, LastPlayedFileError> {
    let safe_variant_name = get_safe_filename(variant.into());
    let directory = data_dir.join("LastPlayed").join(&safe_variant_name);
    create_dir_all(&directory)?;

    let file_path = directory.join("last_played_versions.json");

    Ok(file_path)
}
