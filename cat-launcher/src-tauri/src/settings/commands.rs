use tauri::command;

use cat_macros::CommandErrorSerialize;

use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::settings::fonts::get_all_fonts;
use crate::settings::types::Font;

#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetFontsError {
  #[error("failed to get fonts: {0}")]
  OS(#[from] OSNotSupportedError),
}

#[command]
pub async fn get_fonts() -> Result<Vec<Font>, GetFontsError> {
  let os_str = std::env::consts::OS;
  let os = get_os_enum(os_str)?;
  Ok(get_all_fonts(os).await)
}
