use std::path::Path;
use std::sync::LazyLock;

use downloader::Downloader;
use reqwest::Client;

use crate::infra::consts::{NUM_PARALLEL_DOWNLOADS, USER_AGENT};

pub fn create_downloader(download_folder: &Path) -> downloader::Result<Downloader> {
    let mut builder = Downloader::builder();
    builder
        .download_folder(download_folder)
        .parallel_requests(NUM_PARALLEL_DOWNLOADS);

    builder.build_with_client(HTTP_CLIENT.clone())
}

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to build reqwest client")
});
