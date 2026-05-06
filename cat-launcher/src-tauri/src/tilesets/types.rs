use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

/// Details about the installation of a third-party tileset.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct TilesetInstallation {
  /// The URL where the tileset archive can be downloaded.
  pub download_url: String,
  /// The relative path to the tileset directory within the archive.
  pub tileset: String,
}

/// Information about the activity and origin of a third-party tileset.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct TilesetActivity {
  /// The type of activity or project status.
  pub activity_type: String,
  /// An optional link to the GitHub repository.
  pub github: Option<String>,
}

/// Represents a third-party tileset.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ThirdPartyTileset {
  /// The unique identifier for the tileset.
  pub id: String,
  /// The display name of the tileset.
  pub name: String,
  /// Installation details for the tileset.
  pub installation: TilesetInstallation,
  /// Activity information for the tileset.
  pub activity: TilesetActivity,
}

/// Represents a stock tileset included with the game.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct StockTileset {
  /// The unique identifier for the tileset.
  pub id: String,
  /// The display name of the tileset.
  pub name: String,
}

/// Represents a tileset, which can be either a stock tileset or a third-party one.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "content")]
pub enum Tileset {
  /// A stock tileset.
  Stock(StockTileset),
  /// A third-party tileset.
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

/// The installation status of a third-party tileset.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TilesetInstallationStatus {
  /// The tileset is currently installed.
  Installed,
  /// The tileset is not installed.
  NotInstalled,
}
