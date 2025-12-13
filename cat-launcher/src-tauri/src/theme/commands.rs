use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::State;

use crate::theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository;
use crate::theme::theme::{
  get_theme_preference, update_theme_preference, GetThemeError,
  Theme, ThemePreference, UpdateThemeError,
};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetPreferredThemeCommandError {
  #[error("failed to load theme preference: {0}")]
  Get(#[from] GetThemeError),
}

#[tauri::command]
pub async fn get_preferred_theme(
  repository: State<'_, SqliteThemePreferenceRepository>,
) -> Result<ThemePreference, GetPreferredThemeCommandError> {
  Ok(get_theme_preference(repository.inner()).await?)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum SetPreferredThemeCommandError {
  #[error("failed to update theme preference: {0}")]
  Update(#[from] UpdateThemeError),
}

#[tauri::command]
pub async fn set_preferred_theme(
  theme: Theme,
  repository: State<'_, SqliteThemePreferenceRepository>,
) -> Result<(), SetPreferredThemeCommandError> {
  Ok(update_theme_preference(theme, repository.inner()).await?)
}
