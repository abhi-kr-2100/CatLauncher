pub mod constants;
pub mod filesystem;
pub mod settings;

pub mod active_release;
mod backups;
mod fetch_releases;
mod game_release;
mod game_tips;
mod infra;
mod install_release;
mod launch_game;
mod manual_backups;
mod mods;
mod play_time;
mod theme;
mod users;
mod utils;
mod variants;

use crate::active_release::commands::get_active_release;
use crate::backups::commands::{
    delete_backup_by_id, list_backups_for_variant, restore_backup_by_id,
};
use crate::fetch_releases::commands::fetch_releases_for_variant;
use crate::game_tips::commands::get_tips;
use crate::install_release::commands::install_release;
use crate::install_release::installation_status::commands::get_installation_status;
use crate::launch_game::commands::launch_game;
use crate::manual_backups::commands::{
    create_manual_backup_for_variant, delete_manual_backup_by_id, list_manual_backups_for_variant,
    restore_manual_backup_by_id,
};
use crate::mods::commands::{
    get_third_party_mod_installation_status_command, install_third_party_mod_command,
    list_all_mods_command, uninstall_third_party_mod_command,
};
use crate::play_time::commands::{get_play_time_for_variant, get_play_time_for_version};
use crate::theme::commands::{get_preferred_theme, set_preferred_theme};
use crate::users::commands::get_user_id;
use crate::utils::{
    autoupdate, manage_downloader, manage_http_client, manage_posthog, manage_repositories,
    manage_settings, migrate_backups,
};
use crate::variants::commands::get_game_variants_info;
use crate::variants::commands::update_game_variant_order;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            manage_settings(app)?;
            manage_http_client(app);
            manage_downloader(app);
            manage_repositories(app)?;
            manage_posthog(app);

            migrate_backups(app);

            autoupdate(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_game_variants_info,
            fetch_releases_for_variant,
            install_release,
            launch_game,
            get_active_release,
            get_installation_status,
            get_tips,
            get_play_time_for_variant,
            get_play_time_for_version,
            update_game_variant_order,
            list_backups_for_variant,
            delete_backup_by_id,
            restore_backup_by_id,
            list_manual_backups_for_variant,
            create_manual_backup_for_variant,
            delete_manual_backup_by_id,
            restore_manual_backup_by_id,
            list_all_mods_command,
            install_third_party_mod_command,
            uninstall_third_party_mod_command,
            get_third_party_mod_installation_status_command,
            get_user_id,
            get_preferred_theme,
            set_preferred_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
