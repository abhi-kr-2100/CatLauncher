use tauri::{App, Emitter, Manager, WindowEvent};

pub fn on_quit(app: &App) {
  let app_handle = app.handle().clone();

  // Let the app crash and quit if webview window could not be gotten when quitting
  let window = app.get_webview_window("main").unwrap();

  window.on_window_event(move |event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
      api.prevent_close();
      let _ = app_handle.emit("quit-requested", ());
    }
  });
}
