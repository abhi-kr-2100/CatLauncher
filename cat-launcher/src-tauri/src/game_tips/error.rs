use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use strum::IntoStaticStr;
use thiserror::Error;

use crate::game_tips::game_tips::GetAllTipsForVariantError;
use crate::infra::utils::OSNotSupportedError;

#[derive(Debug, Error, IntoStaticStr)]
pub enum CommandError {
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Failed to get tips: {0}")]
    GetTips(#[from] GetAllTipsForVariantError),

    #[error("OS not supported: {0}")]
    OSNotSupported(#[from] OSNotSupportedError),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommandError", 2)?;
        let error_type: &'static str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
