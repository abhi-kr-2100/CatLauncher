use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Error)]
pub enum ThemeServiceError {
    #[error("Could not get resource directory")]
    ResourceDir(#[source] tauri::Error),
    #[error("Could not get app local data directory")]
    AppLocalDataDir(#[source] tauri::Error),
    #[error("Could not create config directory")]
    CreateConfigDir(#[source] std::io::Error),
    #[error("Could not copy theme file")]
    CopyThemeFile(#[source] std::io::Error),
    #[error("Could not read theme file")]
    ReadThemeFile(#[source] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ThemeService {
    app_handle: tauri::AppHandle,
}

impl ThemeService {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn get_available_themes(&self) -> Result<Vec<Theme>, ThemeServiceError> {
        let mut themes = HashMap::new();
        let resource_dir = resource_dir(&self.app_handle).map_err(ThemeServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        for variant in game_variants.variants {
            let theme_dir = variant.path.join("data").join("raw").join("color_themes");
            if theme_dir.exists() {
                let theme_files = get_all_files_in_directory(&theme_dir).unwrap_or_default();

                for theme_path in theme_files {
                    let theme_name = theme_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace("base_colors-", "");

                    if let std::collections::hash_map::Entry::Vacant(e) =
                        themes.entry(theme_name.clone())
                    {
                        e.insert(Theme {
                            name: theme_name,
                            path: theme_path,
                        });
                    } else {
                        let mut i = 1;
                        loop {
                            let new_name = format!("{} ({})", theme_name, i);
                            if themes.contains_key(&new_name) {
                                i += 1;
                            } else {
                                themes.insert(
                                    new_name.clone(),
                                    Theme {
                                        name: new_name,
                                        path: theme_path,
                                    },
                                );
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut themes_vec: Vec<Theme> = themes.values().cloned().collect();
        themes_vec.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(themes_vec)
    }

    pub fn apply_theme(&self, theme_path: &Path) -> Result<(), ThemeServiceError> {
        let app_local_data_dir = app_local_data_dir(&self.app_handle)
            .ok_or(ThemeServiceError::AppLocalDataDir(tauri::Error::ApiNotAllowlisted("path".to_string())))?;
        let resource_dir = resource_dir(&self.app_handle).map_err(ThemeServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        for variant in game_variants.variants {
            let config_dir = app_local_data_dir
                .join(APP_NAME)
                .join("data")
                .join(variant.id.to_string())
                .join("config");

            if !config_dir.exists() {
                std::fs::create_dir_all(&config_dir).map_err(ThemeServiceError::CreateConfigDir)?;
            }

            let new_theme_path = config_dir.join("base_colors.json");
            std::fs::copy(theme_path, new_theme_path).map_err(ThemeServiceError::CopyThemeFile)?;
        }

        Ok(())
    }

    pub fn get_current_theme(&self) -> Result<Option<Theme>, ThemeServiceError> {
        let app_local_data_dir = app_local_data_dir(&self.app_handle)
            .ok_or(ThemeServiceError::AppLocalDataDir(tauri::Error::ApiNotAllowlisted("path".to_string())))?;
        let resource_dir = resource_dir(&self.app_handle).map_err(ThemeServiceError::ResourceDir)?;
        let game_variants = GameVariants::new(&resource_dir);

        if let Some(variant) = game_variants.variants.first() {
            let base_colors_path = app_local_data_dir
                .join(APP_NAME)
                .join("data")
                .join(variant.id.to_string())
                .join("config")
                .join("base_colors.json");

            if base_colors_path.exists() {
                let base_colors_content =
                    std::fs::read_to_string(&base_colors_path).map_err(ThemeServiceError::ReadThemeFile)?;
                let available_themes = self.get_available_themes()?;

                for theme in available_themes {
                    let theme_content =
                        std::fs::read_to_string(&theme.path).map_err(ThemeServiceError::ReadThemeFile)?;
                    if theme_content == base_colors_content {
                        return Ok(Some(theme));
                    }
                }

                return Ok(Some(Theme {
                    name: "Custom".to_string(),
                    path: base_colors_path,
                }));
            }
        }

        Ok(None)
    }
}
