use std::path::Path;
use std::sync::LazyLock;

use downloader::Downloader;
use reqwest::Client;

pub fn create_downloader(client: Client, download_folder: &Path) -> downloader::Result<Downloader> {
    let mut builder = Downloader::builder();
    builder
        .download_folder(download_folder)
        .parallel_requests(4);

    builder.build_with_client(client)
}

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("cat-launcher")
        .build()
        .expect("Failed to build reqwest client")
});
