use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use font_kit::{family_name::FamilyName, properties::Properties, source::SystemSource};
use heck::ToKebabCase;
use serde::{Deserialize, Serialize};
use tauri_plugin_path::{app_local_data_dir, resource_dir};
use thiserror::Error;
use ts_rs::TS;

use crate::{
    constants::APP_NAME,
    filesystem::utils::get_all_files_in_directory,
    variants::{GameVariant, GameVariants},
};

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq, Eq, Hash)]
#[ts(export)]
pub struct Font {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typeface {
    pub path: String,
    pub hinting: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FontsJson {
    pub typeface: Vec<Typeface>,
    pub gui_typeface: Vec<Typeface>,
    pub map_typeface: Vec<Typeface>,
    pub overmap_typeface: Vec<Typeface>,
}

#[derive(Debug, Error)]
pub enum FontServiceError {
    #[error("Could not get resource directory")]
    ResourceDir(#[source] tauri::Error),
    #[error("Could not get app local data directory")]
    AppLocalDataDir(#[source] tauri::Error),
    #[error("Could not create font directory")]
    CreateFontDir(#[source] std::io::Error),
    #[error("Could not copy font file")]
    CopyFontFile(#[source] std::io::Error),
    #[error("Could not read fonts.json")]
    ReadFontsJson(#[source] std::io::Error),
    #[error("Could not parse fonts.json")]
    ParseFontsJson(#[source] serde_json::Error),
    #[error("Could not write fonts.json")]
    WriteFontsJson(#[source] std::io::Error),
    #[error("Could not get system fonts")]
    SystemFonts,
}

#[derive(Debug, Clone)]
pub struct FontService {
    app_handle: tauri::AppHandle,
}

impl FontService {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn get_monospace_fonts(&self) -> Result<Vec<Font>, FontServiceError> {
        let mut fonts = HashSet::new();
        let source = SystemSource::new();

        let font_paths = source
            .all_fonts()
            .map_err(|_| FontServiceError::SystemFonts)?
            .into_iter()
            .filter(|font| font.is_monospace())
            .map(|font| {
                let name = font.family_name().to_string();
                let path = font.path().to_path_buf();
                Font { name, path }
            })
            .collect::<Vec<Font>>();

        let resource_dir = resource_dir(&self.app_handle).map_err(FontServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        for variant in game_variants.variants {
            let font_dir = variant.path.join("data").join("font");
            if font_dir.exists() {
                let game_fonts = get_all_files_in_directory(&font_dir).unwrap_or_default();

                for font_path in game_fonts {
                    if let Ok(font) = font_kit::font::Font::from_path(&font_path, 0) {
                        if font.is_monospace() {
                            let name = font.family_name().to_string();
                            let path = font_path.clone();
                            fonts.insert(Font { name, path });
                        }
                    }
                }
            }
        }

        fonts.extend(font_paths);

        let mut fonts_vec: Vec<Font> = fonts.into_iter().collect();
        fonts_vec.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(fonts_vec)
    }

    pub fn apply_font(&self, font_name: &str, font_path: &Path) -> Result<(), FontServiceError> {
        let app_local_data_dir = app_local_data_dir(&self.app_handle)
            .ok_or(FontServiceError::AppLocalDataDir(tauri::Error::ApiNotAllowlisted("path".to_string())))?;
        let font_dir = app_local_data_dir.join(APP_NAME).join("font");

        if !font_dir.exists() {
            std::fs::create_dir_all(&font_dir).map_err(FontServiceError::CreateFontDir)?;
        }

        let font_file_name = font_path.file_name().unwrap();
        let new_font_path = font_dir.join(font_file_name);

        std::fs::copy(font_path, &new_font_path).map_err(FontServiceError::CopyFontFile)?;

        let resource_dir = resource_dir(&self.app_handle).map_err(FontServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        for variant in game_variants.variants {
            let fonts_json_path = variant.path.join("config").join("fonts.json");
            let mut fonts_json: FontsJson = if fonts_json_path.exists() {
                let file_content =
                    std::fs::read_to_string(&fonts_json_path).map_err(FontServiceError::ReadFontsJson)?;
                serde_json::from_str(&file_content).map_err(FontServiceError::ParseFontsJson)?
            } else {
                FontsJson {
                    typeface: vec![],
                    gui_typeface: vec![],
                    map_typeface: vec![],
                    overmap_typeface: vec![],
                }
            };

            let new_typeface = Typeface {
                path: format!("font/{}", font_file_name.to_str().unwrap()),
                hinting: "Default".to_string(),
            };

            fonts_json.typeface.insert(0, new_typeface.clone());
            fonts_json.gui_typeface.insert(0, new_typeface.clone());
            fonts_json.map_typeface.insert(0, new_typeface.clone());
            fonts_json.overmap_typeface.insert(0, new_typeface);

            let new_fonts_json_content =
                serde_json::to_string_pretty(&fonts_json).map_err(FontServiceError::ParseFontsJson)?;
            std::fs::write(fonts_json_path, new_fonts_json_content)
                .map_err(FontServiceError::WriteFontsJson)?;
        }

        Ok(())
    }

    pub fn get_current_font(&self) -> Result<Option<Font>, FontServiceError> {
        let resource_dir = resource_dir(&self.app_handle).map_err(FontServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        if let Some(variant) = game_variants.variants.first() {
            let fonts_json_path = variant.path.join("config").join("fonts.json");

            if fonts_json_path.exists() {
                let file_content =
                    std::fs::read_to_string(&fonts_json_path).map_err(FontServiceError::ReadFontsJson)?;
                let fonts_json: FontsJson =
                    serde_json::from_str(&file_content).map_err(FontServiceError::ParseFontsJson)?;

                if let Some(typeface) = fonts_json.typeface.first() {
                    let font_path = if typeface.path.starts_with("font/") {
                        let app_local_data_dir = app_local_data_dir(&self.app_handle)
                            .ok_or(FontServiceError::AppLocalDataDir(tauri::Error::ApiNotAllowlisted("path".to_string())))?;
                        app_local_data_dir
                            .join(APP_NAME)
                            .join(typeface.path.clone())
                    } else {
                        variant.path.join(typeface.path.clone())
                    };

                    if let Ok(font) = font_kit::font::Font::from_path(&font_path, 0) {
                        let name = font.family_name().to_string();
                        return Ok(Some(Font {
                            name,
                            path: font_path,
                        }));
                    }
                }
            }
        }

        Ok(None)
    }
}
