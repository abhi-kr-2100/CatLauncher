use std::path::PathBuf;

use tauri::{command, AppHandle, State};

use crate::themes::theme_service::{Theme, ThemeService};

#[command]
pub fn get_available_themes(theme_service: State<'_, ThemeService>) -> Result<Vec<Theme>, String> {
    theme_service
        .get_available_themes()
        .map_err(|e| e.to_string())
}

#[command]
pub fn apply_theme(theme_path: PathBuf, theme_service: State<'_, ThemeService>) -> Result<(), String> {
    theme_service
        .apply_theme(&theme_path)
        .map_err(|e| e.to_string())
}

#[command]
pub fn get_current_theme(theme_service: State<'_, ThemeService>) -> Result<Option<Theme>, String> {
    theme_service.get_current_theme().map_err(|e| e.to_string())
}

pub fn manage_theme_service(app: &AppHandle) {
    let theme_service = ThemeService::new(app.clone());
    app.manage(theme_service);
}
