use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct TilesetInstallation {
  pub download_url: String,
  pub tileset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct TilesetActivity {
  pub activity_type: String,
  pub github: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartyTileset {
  pub id: String,
  pub name: String,
  pub installation: TilesetInstallation,
  pub activity: TilesetActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockTileset {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Tileset {
  Stock(StockTileset),
  ThirdParty(ThirdPartyTileset),
}

impl Asset for Tileset {
  fn is_third_party(&self) -> bool {
    matches!(self, Tileset::ThirdParty(_))
  }

  fn id(&self) -> &str {
    match self {
      Tileset::Stock(t) => &t.id,
      Tileset::ThirdParty(t) => &t.id,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TilesetInstallationStatus {
  Installed,
  NotInstalled,
}
