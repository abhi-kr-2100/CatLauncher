use std::sync::Mutex;

pub mod constants;
pub mod filesystem;
pub mod settings;
pub mod fonts;
pub mod themes;

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
mod mods;
mod play_time;
mod soundpacks;
mod theme;
mod tilesets;
mod users;
mod utils;
mod variants;

use crate::active_release::commands::get_active_release;
use crate::backups::commands::{
  delete_backup_by_id, list_backups_for_variant, restore_backup_by_id,
};
use crate::fetch_releases::commands::fetch_releases_for_variant;
use crate::fonts::commands::{apply_font, get_current_font, get_monospace_fonts, manage_font_service};
use crate::game_tips::commands::get_tips;
use crate::install_release::commands::install_release;
use crate::install_release::installation_status::commands::get_installation_status;
use crate::last_played_world::commands::get_last_played_world;
use crate::launch_game::commands::launch_game;
use crate::manual_backups::commands::{
  create_manual_backup_for_variant, delete_manual_backup_by_id,
  list_manual_backups_for_variant, restore_manual_backup_by_id,
};
use crate::mods::commands::{
  get_last_activity_on_third_party_mod_command,
  get_third_party_mod_installation_status_command,
  install_third_party_mod_command, list_all_mods_command,
  uninstall_third_party_mod_command,
};
use crate::play_time::commands::{
  get_play_time_for_variant, get_play_time_for_version, log_play_time,
};
use crate::settings::{Settings, SettingsRepository, SqliteSettingsRepository};
use crate::soundpacks::commands::{
  get_third_party_soundpack_installation_status_command,
  install_third_party_soundpack_command, list_all_soundpacks_command,
  uninstall_third_party_soundpack_command,
};
use crate::theme::commands::{
  get_preferred_theme, set_preferred_theme,
};
use crate::themes::commands::{apply_theme, get_available_themes, get_current_theme, manage_theme_service};
use crate::tilesets::commands::{
  get_third_party_tileset_installation_status_command,
  install_third_party_tileset_command, list_all_tilesets_command,
  uninstall_third_party_tileset_command,
};
use crate::users::commands::get_user_id;
use crate::utils::{
  autoupdate, manage_core_services, manage_downloader, manage_http_client, manage_posthog,
  migrate_to_local_data_dir, on_quit,
};
use crate::variants::commands::get_game_variants_info;
use crate::variants::commands::update_game_variant_order;
use tauri::{command, AppHandle, Manager, State};
use crate::infra::download::Downloader;
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::variants::GameVariants;
use tauri_plugin_path::resource_dir;

#[command]
fn confirm_quit(app_handle: AppHandle) {
  app_handle.exit(0)
}

#[command]
async fn get_settings(
  settings_mutex: State<'_, Mutex<Settings>>,
) -> Result<Settings, ()> {
  let settings = settings_mutex.lock().unwrap();
  Ok(settings.clone())
}

#[command]
async fn update_settings(
  settings: Settings,
  settings_mutex: State<'_, Mutex<Settings>>,
  settings_repo: State<'_, SqliteSettingsRepository>,
  backup_repo: State<'_, SqliteBackupRepository>,
  app_handle: AppHandle,
) -> Result<(), ()> {
  let mut current_settings = settings_mutex.lock().unwrap();
  let max_backups = settings.max_backups.get();
  *current_settings = settings.clone();

  let _ = settings_repo.update_settings(&settings).await;

  let resource_dir = resource_dir(&app_handle).unwrap();
  let game_variants = GameVariants::new(&resource_dir);

  for variant in game_variants.variants {
    let backups = backup_repo.get_backups(&variant.id.to_string()).await.unwrap_or_default();
    let backups_to_delete = backups.len().saturating_sub(max_backups);

    if backups_to_delete > 0 {
      for i in 0..backups_to_delete {
        let backup = &backups[i];
        let _ = backup_repo.delete_backup(&backup.id).await;
      }
    }
  }

  let client: State<reqwest::Client> = app_handle.state();
  let downloader = Downloader::new(
    client.inner().clone(),
    settings.parallel_requests,
  );
  app_handle.manage(downloader);

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        if let Err(e) = manage_core_services(&handle).await {
          // TODO: maybe we should exit the app here?
          eprintln!("Error initializing core services: {}", e);
        }
      });

      manage_font_service(app.handle());
      manage_theme_service(app.handle());
      manage_http_client(app);
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
      confirm_quit,
      get_settings,
      update_settings,
      get_monospace_fonts,
      apply_font,
      get_current_font,
      get_available_themes,
      apply_theme,
      get_current_theme,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
