use tauri::{App, Manager};

use crate::infra::http_client::create_http_client;

pub fn manage_http_client(app: &App) {
  let client = create_http_client();
  app.manage(client);
}
