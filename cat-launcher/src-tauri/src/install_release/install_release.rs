use std::path::PathBuf;

use downloader::Downloader;

use crate::game_release::game_release::GameRelease;
use crate::install_release::error::InstallReleaseError;
use crate::install_release::utils::get_asset_download_dir;

impl GameRelease {
    pub async fn install_release(&self) -> Result<PathBuf, InstallReleaseError> {
        let download_dir = get_asset_download_dir(&self.variant)?;
        let mut downloader = Downloader::builder()
            .download_folder(&download_dir)
            .parallel_requests(4)
            .user_agent("cat-launcher")
            .build()?;

        let asset = self.get_asset()?;

        Ok(asset.download(&mut downloader).await?)
    }
}
