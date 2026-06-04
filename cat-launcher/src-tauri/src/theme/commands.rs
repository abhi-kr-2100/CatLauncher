use cat_macros::CommandErrorSerialize;
use tauri::State;

use crate::theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository;
use crate::theme::theme::{
  GetThemeError, Theme, ThemePreference, UpdateThemeError,
  get_theme_preference, update_theme_preference,
};

/// Errors that can occur when retrieving the preferred theme via a command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetPreferredThemeCommandError {
  /// An error occurred while loading the theme preference from the repository.
  #[error("failed to load theme preference: {0}")]
  Get(#[from] GetThemeError),
}

/// Retrieves the user's preferred theme preference.
#[tauri::command]
pub async fn get_preferred_theme(
  repository: State<'_, SqliteThemePreferenceRepository>,
) -> Result<ThemePreference, GetPreferredThemeCommandError> {
  Ok(get_theme_preference(repository.inner()).await?)
}

/// Errors that can occur when setting the preferred theme via a command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum SetPreferredThemeCommandError {
  /// An error occurred while updating the theme preference in the repository.
  #[error("failed to update theme preference: {0}")]
  Update(#[from] UpdateThemeError),
}

/// Sets the user's preferred theme preference.
#[tauri::command]
pub async fn set_preferred_theme(
  theme: Theme,
  repository: State<'_, SqliteThemePreferenceRepository>,
) -> Result<(), SetPreferredThemeCommandError> {
  Ok(update_theme_preference(theme, repository.inner()).await?)
}
