use tauri::{App, Listener};

use crate::infra::autoupdate::update::run_updater;

pub fn autoupdate(app: &App) {
  let handle = app.handle();
  let handle_for_closure = handle.clone();
  handle.once("frontend-ready", move |_event| {
    let handle = handle_for_closure.clone();
    tauri::async_runtime::spawn(async move {
      run_updater(handle).await;
    });
  });
}
