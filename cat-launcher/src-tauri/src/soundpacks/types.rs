use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

/// Details about the installation of a third-party soundpack.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SoundpackInstallation {
  /// The URL where the soundpack archive can be downloaded.
  pub download_url: String,
  /// The relative path to the soundpack directory within the archive.
  pub soundpack: String,
}

/// Information about the activity and origin of a third-party soundpack.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SoundpackActivity {
  /// The type of activity or project status.
  pub activity_type: String,
  /// An optional link to the GitHub repository.
  pub github: Option<String>,
}

/// Represents a third-party soundpack.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartySoundpack {
  /// The unique identifier for the soundpack.
  pub id: String,
  /// The display name of the soundpack.
  pub name: String,
  /// Installation details for the soundpack.
  pub installation: SoundpackInstallation,
  /// Activity information for the soundpack.
  pub activity: SoundpackActivity,
}

/// Represents a stock soundpack included with the game.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockSoundpack {
  /// The unique identifier for the soundpack.
  pub id: String,
  /// The display name of the soundpack.
  pub name: String,
}

/// Represents a soundpack, which can be either a stock soundpack or a third-party one.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Soundpack {
  /// A stock soundpack.
  Stock(StockSoundpack),
  /// A third-party soundpack.
  ThirdParty(ThirdPartySoundpack),
}

impl Asset for Soundpack {
  fn is_third_party(&self) -> bool {
    matches!(self, Soundpack::ThirdParty(_))
  }

  fn id(&self) -> &str {
    match self {
      Soundpack::Stock(s) => &s.id,
      Soundpack::ThirdParty(s) => &s.id,
    }
  }
}

/// The installation status of a third-party soundpack.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SoundpackInstallationStatus {
  /// The soundpack is currently installed.
  Installed,
  /// The soundpack is not installed.
  NotInstalled,
}
