use coded_error::CodedError;
use strum::IntoStaticStr;
use thiserror::Error;

use crate::game_tips::game_tips::GetAllTipsForVariantError;
use crate::infra::utils::OSNotSupportedError;

#[derive(Debug, Error, IntoStaticStr, CodedError)]
pub enum CommandError {
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Failed to get tips: {0}")]
    GetTips(#[from] GetAllTipsForVariantError),

    #[error("OS not supported: {0}")]
    OSNotSupported(#[from] OSNotSupportedError),
}
