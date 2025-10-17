use std::io;
use std::path::Path;

use serde::de::DeserializeOwned;
use tokio::fs;

use crate::variants::GameVariant;

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

pub async fn read_from_file<T: DeserializeOwned>(path: &Path) -> Result<T, ReadFromFileError> {
    let contents = fs::read_to_string(path).await?;
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

pub async fn write_to_file<T: serde::Serialize>(
    path: &Path,
    data: &T,
) -> Result<(), WriteToFileError> {
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(path, contents).await?;
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum OS {
    Linux,
    Windows,
    MacOS,
}

#[derive(Debug, thiserror::Error)]
#[error("OS not supported: {os}")]
pub struct OSNotSupportedError {
    os: &'static str,
}

pub fn get_os_enum(os: &'static str) -> Result<OS, OSNotSupportedError> {
    match os {
        "linux" => Ok(OS::Linux),
        "windows" => Ok(OS::Windows),
        "macos" => Ok(OS::MacOS),
        _ => Err(OSNotSupportedError { os }),
    }
}

#[derive(Debug, PartialEq)]
pub enum Arch {
    ARM64,
    X64,
}

#[derive(Debug, thiserror::Error)]
#[error("Architecture not supported: {arch}")]
pub struct ArchNotSupportedError {
    arch: &'static str,
}

pub fn get_arch_enum(arch: &'static str) -> Result<Arch, ArchNotSupportedError> {
    match arch {
        "aarch64" => Ok(Arch::ARM64),
        "x86_64" => Ok(Arch::X64),
        _ => Err(ArchNotSupportedError { arch }),
    }
}
