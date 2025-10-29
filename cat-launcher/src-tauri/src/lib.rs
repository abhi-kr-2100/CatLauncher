pub mod basic_info;
pub mod filesystem;
pub mod settings;

mod fetch_releases;
mod game_release;
mod game_tips;
mod infra;
mod install_release;
mod last_played;
mod launch_game;
mod play_time;
mod utils;
mod variants;

use crate::basic_info::commands::get_game_variants_info;
use crate::fetch_releases::commands::fetch_releases_for_variant;
use crate::game_tips::commands::get_tips;
use crate::install_release::commands::install_release;
use crate::install_release::installation_status::commands::get_installation_status;
use crate::last_played::commands::get_last_played_version;
use crate::launch_game::commands::launch_game;
use crate::play_time::commands::{get_play_time_for_variant, get_play_time_for_version};
use crate::utils::{autoupdate, manage_repositories, manage_settings};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            manage_settings(app)?;
            manage_repositories(app)?;
            autoupdate(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_game_variants_info,
            fetch_releases_for_variant,
            install_release,
            launch_game,
            get_last_played_version,
            get_installation_status,
            get_tips,
            get_play_time_for_variant,
            get_play_time_for_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
