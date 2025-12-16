use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ModInstallation {
  pub download_url: String,
  pub modinfo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ModActivity {
  pub activity_type: String,
  pub github: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartyMod {
  pub id: String,
  pub name: String,
  pub description: String,
  pub category: String,
  pub installation: ModInstallation,
  pub activity: ModActivity,
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
