use std::path::Path;
use std::{fs, io};

use serde::de::DeserializeOwned;

use crate::variants::GameVariant;

pub fn get_safe_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn get_github_repo_for_variant(variant: &GameVariant) -> &'static str {
    match variant {
        GameVariant::DarkDaysAhead => "CleverRaven/Cataclysm-DDA",
        GameVariant::BrightNights => "cataclysmbnteam/Cataclysm-BN",
        GameVariant::TheLastGeneration => "Cataclysm-TLG/Cataclysm-TLG",
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReadFromFileError {
    #[error("failed to read from file: {0}")]
    Read(#[from] io::Error),

    #[error("failed to deserialize data: {0}")]
    Deserialize(#[from] serde_json::Error),
}

pub fn read_from_file<T: DeserializeOwned>(path: &Path) -> Result<T, ReadFromFileError> {
    let contents = fs::read_to_string(path)?;
    let v = serde_json::from_str(&contents)?;
    Ok(v)
}

#[derive(thiserror::Error, Debug)]
pub enum WriteToFileError {
    #[error("failed to serialize data: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("failed to write to file: {0}")]
    Write(#[from] std::io::Error),
}

pub fn write_to_file<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), WriteToFileError> {
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(path, contents)?;
    Ok(())
}
