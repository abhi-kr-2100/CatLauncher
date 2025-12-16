use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SoundpackInstallation {
  pub download_url: String,
  pub soundpack: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SoundpackActivity {
  pub activity_type: String,
  pub github: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartySoundpack {
  pub id: String,
  pub name: String,
  pub installation: SoundpackInstallation,
  pub activity: SoundpackActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockSoundpack {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Soundpack {
  Stock(StockSoundpack),
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SoundpackInstallationStatus {
  Installed,
  NotInstalled,
}
