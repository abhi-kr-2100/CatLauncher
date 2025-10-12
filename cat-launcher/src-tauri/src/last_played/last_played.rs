use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::filesystem::paths::{get_last_played_filepath, LastPlayedFileError};
use crate::infra::utils::{read_from_file, write_to_file, ReadFromFileError, WriteToFileError};
use crate::variants::GameVariant;

#[derive(Debug, Serialize, Deserialize)]
pub struct LastPlayedData {
    pub versions: HashMap<String, String>,
}

#[derive(thiserror::Error, Debug)]
pub enum LastPlayedError {
    #[error("failed to read last played data: {0}")]
    Read(#[from] ReadFromFileError),

    #[error("failed to write last played data: {0}")]
    Write(#[from] WriteToFileError),

    #[error("failed to get last played file path: {0}")]
    GetFilePath(#[from] LastPlayedFileError),
}

impl GameVariant {
    pub async fn get_last_played_version(
        &self,
        data_dir: &Path,
    ) -> Result<Option<String>, LastPlayedError> {
        let file_path = get_last_played_filepath(self, data_dir).await?;

        match fs::metadata(&file_path).await {
            Ok(metadata) if metadata.is_file() => {}
            _ => return Ok(None),
        };

        let mut data: LastPlayedData = read_from_file(&file_path).await?;
        let variant_key: &'static str = self.into();

        Ok(data.versions.remove(variant_key))
    }

    pub async fn set_last_played_version(
        &self,
        version: &str,
        data_dir: &Path,
    ) -> Result<(), LastPlayedError> {
        let file_path = get_last_played_filepath(self, data_dir).await?;

        let mut data = match fs::metadata(&file_path).await {
            Ok(metadata) if metadata.is_file() => read_from_file(&file_path).await?,
            _ => LastPlayedData {
                versions: std::collections::HashMap::new(),
            },
        };

        let variant_key: &'static str = self.into();

        data.versions
            .insert(variant_key.into(), version.to_string());

        write_to_file(&file_path, &data).await?;

        Ok(())
    }
}
