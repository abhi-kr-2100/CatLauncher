use std::path::{Path, PathBuf};

use crate::game_release::game_release::GameRelease;
use crate::infra::http_client::{create_downloader, HTTP_CLIENT};
use crate::install_release::error::InstallReleaseError;
use crate::install_release::utils::get_asset_download_dir;

impl GameRelease {
    pub async fn install_release(
        &self,
        cache_dir: &Path,
        data_dir: &Path,
    ) -> Result<PathBuf, InstallReleaseError> {
        let download_dir = get_asset_download_dir(&self.variant, data_dir)?;

        let http_client = match &*HTTP_CLIENT {
            Ok(client) => client,
            Err(err) => return Err(err.into()),
        };
        let mut downloader = create_downloader(&download_dir, &http_client)?;

        let asset = self.get_asset(cache_dir)?;

        Ok(asset.download(&mut downloader).await?)
    }
}
