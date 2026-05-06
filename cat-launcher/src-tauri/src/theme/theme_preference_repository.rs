use async_trait::async_trait;

use crate::theme::theme::{Theme, ThemePreference};

/// Errors that can occur when interacting with a theme preference repository.
#[derive(Debug, thiserror::Error)]
pub enum ThemePreferenceRepositoryError {
  /// An error occurred while reading the theme preference.
  #[error("failed to read theme preference: {0}")]
  Get(#[source] Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while persisting the theme preference.
  #[error("failed to persist theme preference: {0}")]
  Update(#[source] Box<dyn std::error::Error + Send + Sync>),

  /// The theme value retrieved from the repository is invalid.
  #[error("invalid theme value: {0}")]
  InvalidTheme(String),
}

/// A repository for managing user theme preferences.
#[async_trait]
pub trait ThemePreferenceRepository: Send + Sync {
  /// Retrieves the currently preferred theme.
  async fn get_preferred_theme(
    &self,
  ) -> Result<ThemePreference, ThemePreferenceRepositoryError>;

  /// Sets and persists the preferred theme.
  async fn set_preferred_theme(
    &self,
    theme: &Theme,
  ) -> Result<(), ThemePreferenceRepositoryError>;
}
