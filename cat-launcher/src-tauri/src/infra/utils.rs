use std::io;
use std::path::Path;

use serde::de::DeserializeOwned;
use tokio::fs;

use crate::variants::GameVariant;

/// A trait for assets in the launcher.
pub trait Asset {
  /// Returns `true` if the asset is third-party.
  fn is_third_party(&self) -> bool;

  /// Returns the ID of the asset.
  fn id(&self) -> &str;
}

/// Returns the GitHub repository string for a given game variant.
pub fn get_github_repo_for_variant(
  variant: &GameVariant,
) -> &'static str {
  match variant {
    GameVariant::DarkDaysAhead => "CleverRaven/Cataclysm-DDA",
    GameVariant::BrightNights => "cataclysmbnteam/Cataclysm-BN",
    GameVariant::TheLastGeneration => "Cataclysm-TLG/Cataclysm-TLG",
  }
}

/// Represents errors that can occur while reading and deserializing data from a file.
#[derive(thiserror::Error, Debug)]
pub enum ReadFromFileError {
  #[error("failed to read from file: {0}")]
  Read(#[from] io::Error),

  #[error("failed to deserialize data: {0}")]
  Deserialize(#[from] serde_json::Error),
}

/// Reads the content of a file at the given path and deserializes it to type `T`.
pub async fn read_from_file<T: DeserializeOwned>(
  path: &Path,
) -> Result<T, ReadFromFileError> {
  let contents = fs::read_to_string(path).await?;
  let v = serde_json::from_str(&contents)?;
  Ok(v)
}

#[derive(Debug, PartialEq)]
pub enum OS {
  Linux,
  Windows,
  Mac,
}

#[derive(Debug, thiserror::Error)]
#[error("OS not supported: {os}")]
pub struct OSNotSupportedError {
  os: &'static str,
}

pub fn get_os_enum(
  os: &'static str,
) -> Result<OS, OSNotSupportedError> {
  match os {
    "linux" => Ok(OS::Linux),
    "windows" => Ok(OS::Windows),
    "macos" => Ok(OS::Mac),
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

pub fn get_arch_enum(
  arch: &'static str,
) -> Result<Arch, ArchNotSupportedError> {
  match arch {
    "aarch64" => Ok(Arch::ARM64),
    "x86_64" => Ok(Arch::X64),
    _ => Err(ArchNotSupportedError { arch }),
  }
}

#[derive(Debug, PartialEq)]
pub struct HostSystem {
  pub os: OS,
  pub arch: Arch,
}

#[derive(Debug, thiserror::Error)]
pub enum HostSystemError {
  #[error("OS not supported: {0}")]
  Os(#[from] OSNotSupportedError),

  #[error("Architecture not supported: {0}")]
  Arch(#[from] ArchNotSupportedError),
}

impl HostSystem {
  pub fn current(
    os: &'static str,
    arch: &'static str,
  ) -> Result<Self, HostSystemError> {
    Ok(Self {
      os: get_os_enum(os)?,
      arch: get_arch_enum(arch)?,
    })
  }
}

pub fn sort_assets<T: Asset>(items: &mut [T]) {
  items.sort_by(|a, b| {
    a.is_third_party()
      .cmp(&b.is_third_party())
      .reverse()
      .then_with(|| a.id().cmp(b.id()))
  });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_current_parses_supported_platform() {
    assert_eq!(
      HostSystem::current("linux", "x86_64").ok(),
      Some(HostSystem {
        os: OS::Linux,
        arch: Arch::X64,
      })
    );

    assert_eq!(
      HostSystem::current("macos", "aarch64").ok(),
      Some(HostSystem {
        os: OS::Mac,
        arch: Arch::ARM64,
      })
    );

    assert_eq!(
      HostSystem::current("windows", "x86_64").ok(),
      Some(HostSystem {
        os: OS::Windows,
        arch: Arch::X64,
      })
    );
  }

  #[test]
  fn test_current_rejects_unsupported_os() {
    assert!(matches!(
      HostSystem::current("freebsd", "x86_64"),
      Err(HostSystemError::Os(_))
    ));
  }

  #[test]
  fn test_current_rejects_unsupported_arch() {
    assert!(matches!(
      HostSystem::current("linux", "wasm32"),
      Err(HostSystemError::Arch(_))
    ));
  }
}
