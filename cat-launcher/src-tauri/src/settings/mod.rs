pub mod apply_settings;
pub mod commands;
pub mod list_fonts;
pub mod list_themes;
#[allow(clippy::module_inception)]
pub mod settings;
pub mod types;

pub use settings::Settings;
pub use settings::SettingsData;
pub use types::{
  ColorDefWrapper, ColorTheme, Font, FontsConfig, GameSettings,
  ThemeColors,
};
