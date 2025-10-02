use std::path::Path;
use std::sync::LazyLock;

use downloader::{Downloader, Result as DownloaderResult};
use reqwest::{Client, Error as ReqwestError};

use crate::infra::consts::{NUM_PARALLEL_DOWNLOADS, USER_AGENT};

pub fn create_downloader(
    download_folder: &Path,
    http_client: &Client,
) -> DownloaderResult<Downloader> {
    let mut builder = Downloader::builder();
    builder
        .download_folder(download_folder)
        .parallel_requests(NUM_PARALLEL_DOWNLOADS);

    builder.build_with_client(http_client.clone())
}

pub static HTTP_CLIENT: LazyLock<Result<Client, ReqwestError>> =
    LazyLock::new(|| Client::builder().user_agent(USER_AGENT).build());
