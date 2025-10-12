pub mod filesystem;

mod basic_info;
mod fetch_releases;
mod game_release;
mod infra;
mod install_release;
mod last_played;
mod launch_game;
mod variants;

use tauri_plugin_updater::UpdaterExt;

use crate::basic_info::commands::get_game_variants_info;
use crate::fetch_releases::commands::fetch_releases_for_variant;
use crate::install_release::commands::install_release;
use crate::install_release::installation_status::commands::get_installation_status;
use crate::last_played::commands::get_last_played_version;
use crate::launch_game::commands::launch_game;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(updater) = handle.updater() {
                    match updater.check().await {
                        Ok(Some(update)) => {
                            println!("Update available: {}", update.version);
                            if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                                println!("Failed to install update: {}", e);
                            } else {
                                println!("Update installed");
                            }
                        }
                        Ok(None) => {
                            println!("No update available");
                        }
                        Err(e) => {
                            println!("Failed to check for updates: {}", e);
                        }
                    }
                } else {
                    println!("Updater not available");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_game_variants_info,
            fetch_releases_for_variant,
            install_release,
            launch_game,
            get_last_played_version,
            get_installation_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
