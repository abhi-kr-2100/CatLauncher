use tauri::{App, Manager};

use crate::{
  constants::PARALLEL_REQUESTS, infra::download::Downloader,
};

pub fn manage_downloader(app: &App) {
  let client: tauri::State<reqwest::Client> = app.state();
  let downloader =
    Downloader::new(client.inner().clone(), PARALLEL_REQUESTS);
  app.manage(downloader);
}
