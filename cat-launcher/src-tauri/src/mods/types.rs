//! Types related to game mods.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;
pub use crate::mods::online::types::{
  FetchOnlineModsError, OnlineModRepository,
};
use crate::variants::GameVariant;

/// Represents mod installation information.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ModInstallation {
  /// The URL to download the mod from.
  pub download_url: String,
  /// The mod information JSON or similar structure.
  pub modinfo: String,
}

/// Represents the type of activity associated with a mod.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "activity_type")]
pub enum ModActivity {
  /// GitHub repository commit information.
  #[serde(rename = "github_commit")]
  GithubCommit { github: String },
}

/// Represents a third-party mod.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartyMod {
  /// Unique identifier for the mod.
  pub id: String,
  /// Name of the mod.
  pub name: String,
  /// Description of the mod.
  pub description: String,
  /// Category the mod belongs to.
  pub category: String,
  /// Installation details.
  pub installation: ModInstallation,
  /// Optional activity metadata.
  pub activity: Option<ModActivity>,
}

/// Represents a stock mod included in the game release.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockMod {
  /// Unique identifier for the mod.
  pub id: String,
  /// Name of the mod.
  pub name: String,
  /// Description of the mod.
  pub description: String,
  /// Category the mod belongs to.
  pub category: String,
}

/// Enumerates the different types of mods.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Mod {
  /// A built-in stock mod.
  Stock(StockMod),
  /// A third-party community mod.
  ThirdParty(ThirdPartyMod),
}

impl Asset for Mod {
  fn is_third_party(&self) -> bool {
    matches!(self, Mod::ThirdParty(_))
  }

  fn id(&self) -> &str {
    match self {
      Mod::Stock(m) => &m.id,
      Mod::ThirdParty(m) => &m.id,
    }
  }
}

/// Represents the installation status of a mod.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ModInstallationStatus {
  /// The mod is currently installed.
  Installed,
  /// The mod is not installed.
  NotInstalled,
}

/// Payload sent when updating mod information.
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ModsUpdatePayload {
  /// The game variant the updates pertain to.
  pub variant: GameVariant,
  /// The list of mods.
  pub mods: Vec<Mod>,
  /// The status of the update process.
  pub status: ModsUpdateStatus,
}

/// Represents the status of a mod update operation.
#[derive(Debug, Clone, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ModsUpdateStatus {
  /// The mods are currently being fetched.
  Fetching,
  /// The fetch operation succeeded.
  Success,
  /// Reserved for future use when errors are streamed via the update channel
  /// instead of being returned as a command Result.
  #[allow(dead_code)]
  Error,
}
