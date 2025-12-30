use std::path::PathBuf;

use tauri::{command, AppHandle, State};

use crate::fonts::font_service::{Font, FontService};

#[command]
pub fn get_monospace_fonts(font_service: State<'_, FontService>) -> Result<Vec<Font>, String> {
    font_service
        .get_monospace_fonts()
        .map_err(|e| e.to_string())
}

#[command]
pub fn apply_font(
    font_name: String,
    font_path: PathBuf,
    font_service: State<'_, FontService>,
) -> Result<(), String> {
    font_service
        .apply_font(&font_name, &font_path)
        .map_err(|e| e.to_string())
}

#[command]
pub fn get_current_font(font_service: State<'_, FontService>) -> Result<Option<Font>, String> {
    font_service.get_current_font().map_err(|e| e.to_string())
}

pub fn manage_font_service(app: &AppHandle) {
    let font_service = FontService::new(app.clone());
    app.manage(font_service);
}
