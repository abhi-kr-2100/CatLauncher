use std::fs::create_dir_all;
use std::io::Error as IoError;
use std::path::PathBuf;

use crate::infra::utils::get_safe_filename;
use crate::variants::GameVariant;

pub(crate) fn get_asset_download_dir(variant: &GameVariant) -> Result<PathBuf, IoError> {
    let safe_variant_name = get_safe_filename(variant.into());

    let dir = PathBuf::from("CatLauncherCache")
        .join("Releases")
        .join("Assets")
        .join(&safe_variant_name);

    create_dir_all(&dir)?;

    Ok(dir)
}
