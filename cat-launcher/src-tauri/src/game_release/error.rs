use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Serialize)]
pub enum GameReleaseError {
    #[error("No compatible asset was found in the given release")]
    NoCompatibleAssetFound,
}
