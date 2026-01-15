use tauri::{App, Manager};

use crate::settings::repository::{
  settings_repository::{GetSettingsError, SettingsRepository},
  sqlite_settings_repository::SqliteSettingsRepository,
};

#[derive(thiserror::Error, Debug)]
pub enum ManageSettingsError {
  #[error("failed to get settings: {0}")]
  Get(#[from] GetSettingsError),
}

pub fn manage_settings(app: &App) -> Result<(), ManageSettingsError> {
  let settings_repo = app.state::<SqliteSettingsRepository>();
  let settings =
    tauri::async_runtime::block_on(settings_repo.get_settings())?;

  app.manage(settings);

  Ok(())
}
