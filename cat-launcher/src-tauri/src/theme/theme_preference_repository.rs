use async_trait::async_trait;

use crate::theme::theme::{Theme, ThemePreference};

#[derive(Debug, thiserror::Error)]
pub enum ThemePreferenceRepositoryError {
    #[error("failed to read theme preference: {0}")]
    Get(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to persist theme preference: {0}")]
    Update(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid theme value: {0}")]
    InvalidTheme(String),
}

#[async_trait]
pub trait ThemePreferenceRepository: Send + Sync {
    async fn get_preferred_theme(&self) -> Result<ThemePreference, ThemePreferenceRepositoryError>;

    async fn set_preferred_theme(&self, theme: &Theme)
        -> Result<(), ThemePreferenceRepositoryError>;
}
