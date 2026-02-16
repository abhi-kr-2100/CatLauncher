use strum::IntoStaticStr;
use tauri::{command, AppHandle};

use cat_macros::CommandErrorSerialize;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ConfirmQuitCommandError {
  // Add variants if there are possible errors, but for now it's simple
}

#[command]
pub fn confirm_quit(
  app_handle: AppHandle,
) -> Result<(), ConfirmQuitCommandError> {
  app_handle.exit(0);
  Ok(())
}
