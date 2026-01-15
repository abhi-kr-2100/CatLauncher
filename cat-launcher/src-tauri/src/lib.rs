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
mod last_played_world;
mod launch_game;
mod manual_backups;
mod master_reset;
mod mods;
mod play_time;
mod setup;
mod soundpacks;
mod theme;
mod tilesets;
mod users;
mod variants;

use crate::active_release::commands::get_active_release;
use crate::backups::commands::{
  delete_backup_by_id, list_backups_for_variant, restore_backup_by_id,
};
use crate::fetch_releases::commands::{
  fetch_release_notes, fetch_releases_for_variant,
};
use crate::game_tips::commands::get_tips;
use crate::install_release::commands::install_release;
use crate::install_release::installation_status::commands::get_installation_status;
use crate::last_played_world::commands::get_last_played_world;
use crate::launch_game::commands::launch_game;
use crate::manual_backups::commands::{
  create_manual_backup_for_variant, delete_manual_backup_by_id,
  list_manual_backups_for_variant, restore_manual_backup_by_id,
};
use crate::master_reset::commands::master_reset;
use crate::mods::commands::{
  get_last_activity_on_third_party_mod_command,
  get_third_party_mod_installation_status_command,
  install_third_party_mod_command, list_all_mods_command,
  uninstall_third_party_mod_command,
};
use crate::play_time::commands::{
  get_play_time_for_variant, get_play_time_for_version, log_play_time,
};
use crate::settings::commands::{
  get_color_themes, get_default_settings, get_fonts, get_settings,
  update_settings,
};
use crate::setup::{
  autoupdate::autoupdate, commands::on_quit,
  database::manage_repositories, downloader::manage_downloader,
  http_client::manage_http_client,
  migrations::migrate_to_local_data_dir,
  online_mod_repository_registry::manage_online_mod_repository_registry,
  posthog::manage_posthog, settings::manage_settings,
};
use crate::soundpacks::commands::{
  get_third_party_soundpack_installation_status_command,
  install_third_party_soundpack_command, list_all_soundpacks_command,
  uninstall_third_party_soundpack_command,
};
use crate::theme::commands::{
  get_preferred_theme, set_preferred_theme,
};
use crate::tilesets::commands::{
  get_third_party_tileset_installation_status_command,
  install_third_party_tileset_command, list_all_tilesets_command,
  uninstall_third_party_tileset_command,
};
use crate::users::commands::get_user_id;
use crate::variants::commands::get_game_variants_info;
use crate::variants::commands::update_game_variant_order;
use tauri::{command, AppHandle};

#[command]
fn confirm_quit(app_handle: AppHandle) {
  app_handle.exit(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      manage_http_client(app);
      manage_repositories(app)?;
      manage_settings(app)?;
      manage_online_mod_repository_registry(app);
      manage_downloader(app);
      manage_posthog(app);

      migrate_to_local_data_dir(app);

      autoupdate(app);
      on_quit(app);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_game_variants_info,
      fetch_releases_for_variant,
      fetch_release_notes,
      install_release,
      launch_game,
      get_active_release,
      get_installation_status,
      get_tips,
      get_play_time_for_variant,
      get_play_time_for_version,
      log_play_time,
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
      get_last_activity_on_third_party_mod_command,
      list_all_tilesets_command,
      install_third_party_tileset_command,
      uninstall_third_party_tileset_command,
      get_third_party_tileset_installation_status_command,
      list_all_soundpacks_command,
      install_third_party_soundpack_command,
      uninstall_third_party_soundpack_command,
      get_third_party_soundpack_installation_status_command,
      get_user_id,
      get_preferred_theme,
      set_preferred_theme,
      get_last_played_world,
      get_fonts,
      get_color_themes,
      get_settings,
      update_settings,
      get_default_settings,
      confirm_quit,
      master_reset,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
