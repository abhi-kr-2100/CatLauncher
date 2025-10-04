use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::infra::utils::{read_from_file, write_to_file, ReadFromFileError, WriteToFileError};
use crate::last_played::utils::{get_last_played_file_path, LastPlayedFileError};
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
    pub fn get_last_played_version(
        &self,
        data_dir: &Path,
    ) -> Result<Option<String>, LastPlayedError> {
        let file_path = get_last_played_file_path(self, data_dir)?;

        if !file_path.exists() {
            return Ok(None);
        }

        let mut data: LastPlayedData = read_from_file(&file_path)?;
        let variant_key: &'static str = self.into();

        Ok(data.versions.remove(variant_key))
    }

    pub fn set_last_played_version(
        &self,
        version: &str,
        data_dir: &Path,
    ) -> Result<(), LastPlayedError> {
        let file_path = get_last_played_file_path(self, data_dir)?;

        let mut data = if file_path.exists() {
            read_from_file(&file_path)?
        } else {
            LastPlayedData {
                versions: std::collections::HashMap::new(),
            }
        };

        let variant_key: &'static str = self.into();

        data.versions
            .insert(variant_key.into(), version.to_string());

        write_to_file(&file_path, &data)?;

        Ok(())
    }
}
