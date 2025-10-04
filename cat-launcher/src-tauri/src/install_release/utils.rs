use std::fs::create_dir_all;
use std::io;
use std::path::{Path, PathBuf};

use crate::infra::utils::get_safe_filename;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum AssetDownloadDirError {
    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),
}

pub fn get_asset_download_dir(
    variant: &GameVariant,
    data_dir: &Path,
) -> Result<PathBuf, AssetDownloadDirError> {
    let safe_variant_name = get_safe_filename(variant.into());

    let dir = data_dir
        .join("Releases")
        .join("Assets")
        .join(&safe_variant_name);

    create_dir_all(&dir)?;

    Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum AssetExtractionDirError {
    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),
}

pub fn get_asset_extraction_dir(
    release_version: &str,
    download_dir: &Path,
) -> Result<PathBuf, AssetExtractionDirError> {
    let safe_dir_name = get_safe_filename(&release_version);
    let dir = download_dir.join(&safe_dir_name);

    create_dir_all(&dir)?;

    Ok(dir)
}
