use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;
pub use crate::mods::online::types::{
  FetchOnlineModsError, OnlineModRepository,
};
use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ModInstallation {
  pub download_url: String,
  pub modinfo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "activity_type")]
pub enum ModActivity {
  #[serde(rename = "github_commit")]
  GithubCommit { github: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartyMod {
  pub id: String,
  pub name: String,
  pub description: String,
  pub category: String,
  pub installation: ModInstallation,
  pub activity: Option<ModActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockMod {
  pub id: String,
  pub name: String,
  pub description: String,
  pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Mod {
  Stock(StockMod),
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ModInstallationStatus {
  Installed,
  NotInstalled,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ModsUpdatePayload {
  pub variant: GameVariant,
  pub mods: Vec<Mod>,
  pub status: ModsUpdateStatus,
}

#[derive(Debug, Clone, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ModsUpdateStatus {
  Fetching,
  Success,
  /// Reserved for future use when errors are streamed via the update channel
  /// instead of being returned as a command Result.
  #[allow(dead_code)]
  Error,
}
