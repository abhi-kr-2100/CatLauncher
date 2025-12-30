#[allow(clippy::module_inception)]
pub mod settings;
pub mod repository;

pub use settings::Settings;
pub use repository::{SettingsRepository, SqliteSettingsRepository, SettingsRepositoryError};
