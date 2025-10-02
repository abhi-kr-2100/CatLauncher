mod basic_info;
mod fetch_releases;
mod game_release;
mod infra;
mod variants;

use crate::basic_info::commands::get_game_variants_info;
use crate::fetch_releases::commands::fetch_releases_for_variant;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_game_variants_info,
            fetch_releases_for_variant
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
