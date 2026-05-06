use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::theme::theme_preference_repository::{
  ThemePreferenceRepository, ThemePreferenceRepositoryError,
};

/// Represents the available UI themes.
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Serialize,
  Deserialize,
  TS,
  Display,
  EnumString,
  IntoStaticStr,
  EnumIter,
)]
#[strum(ascii_case_insensitive)]
#[ts(export)]
pub enum Theme {
  /// Light theme.
  Light,
  /// Dark theme.
  Dark,
}

/// Represents the user's theme preference.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThemePreference {
  /// The selected theme.
  pub theme: Theme,
}

/// Errors that can occur when retrieving the theme preference.
#[derive(Debug, thiserror::Error)]
pub enum GetThemeError {
  /// An error occurred in the underlying repository.
  #[error("failed to load theme preference: {0}")]
  Repository(#[from] ThemePreferenceRepositoryError),
}

/// Retrieves the current theme preference from the repository.
pub async fn get_theme_preference(
  repository: &impl ThemePreferenceRepository,
) -> Result<ThemePreference, GetThemeError> {
  Ok(repository.get_preferred_theme().await?)
}

/// Errors that can occur when updating the theme preference.
#[derive(Debug, thiserror::Error)]
pub enum UpdateThemeError {
  /// An error occurred in the underlying repository.
  #[error("failed to update theme preference: {0}")]
  Repository(#[from] ThemePreferenceRepositoryError),
}

/// Updates the theme preference in the repository.
pub async fn update_theme_preference(
  theme: Theme,
  repository: &impl ThemePreferenceRepository,
) -> Result<(), UpdateThemeError> {
  repository.set_preferred_theme(&theme).await?;
  Ok(())
}
