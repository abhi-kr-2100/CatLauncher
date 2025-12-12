use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::theme::theme_preference_repository::{
    ThemePreferenceRepository,
    ThemePreferenceRepositoryError,
};

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
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThemePreference {
    pub theme: Theme,
}

#[derive(Debug, thiserror::Error)]
pub enum GetThemeError {
    #[error("failed to load theme preference: {0}")]
    Repository(#[from] ThemePreferenceRepositoryError),
}

pub async fn get_theme_preference(
    repository: &impl ThemePreferenceRepository,
) -> Result<ThemePreference, GetThemeError> {
    Ok(repository.get_preferred_theme().await?)
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateThemeError {
    #[error("failed to update theme preference: {0}")]
    Repository(#[from] ThemePreferenceRepositoryError),
}

pub async fn update_theme_preference(
    theme: Theme,
    repository: &impl ThemePreferenceRepository,
) -> Result<(), UpdateThemeError> {
    repository.set_preferred_theme(&theme).await?;
    Ok(())
}
