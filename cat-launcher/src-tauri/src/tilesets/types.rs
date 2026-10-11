use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TilesetInstallation {
  pub download_url: String,

  pub tileset: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TilesetActivity {
  pub activity_type: String,

  pub github: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThirdPartyTileset {
  pub id: String,

  pub name: String,

  pub installation: TilesetInstallation,

  pub activity: TilesetActivity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StockTileset {
  pub id: String,

  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TilesetInstallationStatus {
  Installed,

  NotInstalled,
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[test]
  fn test_tileset_asset_trait_stock() {
    let stock = StockTileset {
      id: "ChestHole".to_string(),
      name: "ChestHole Tileset".to_string(),
    };
    let tileset = Tileset::Stock(stock);

    assert!(!tileset.is_third_party());
    assert_eq!(tileset.id(), "ChestHole");
  }

  #[test]
  fn test_tileset_asset_trait_third_party() {
    let tp = ThirdPartyTileset {
      id: "undeadpeople".to_string(),
      name: "UndeadPeople Tileset".to_string(),
      installation: TilesetInstallation {
        download_url: "https://example.com/tileset.zip".to_string(),
        tileset: "tileset.txt".to_string(),
      },
      activity: TilesetActivity {
        activity_type: "github_commit".to_string(),
        github: Some("owner/repo".to_string()),
      },
    };
    let tileset = Tileset::ThirdParty(tp);

    assert!(tileset.is_third_party());
    assert_eq!(tileset.id(), "undeadpeople");
  }

  #[test]
  fn test_tileset_serde_roundtrip() -> TestResult {
    let stock = Tileset::Stock(StockTileset {
      id: "basic".to_string(),
      name: "Basic Tileset".to_string(),
    });

    let json = serde_json::to_string(&stock)?;
    let deserialized: Tileset = serde_json::from_str(&json)?;
    assert_eq!(stock, deserialized);

    let tp = Tileset::ThirdParty(ThirdPartyTileset {
      id: "tp-tileset".to_string(),
      name: "Third Party Tileset".to_string(),
      installation: TilesetInstallation {
        download_url: "https://example.com/tp.zip".to_string(),
        tileset: "gfx/tileset.txt".to_string(),
      },
      activity: TilesetActivity {
        activity_type: "github_release".to_string(),
        github: None,
      },
    });

    let json_tp = serde_json::to_string(&tp)?;
    let deserialized_tp: Tileset = serde_json::from_str(&json_tp)?;
    assert_eq!(tp, deserialized_tp);

    let status_installed = TilesetInstallationStatus::Installed;
    let json_status = serde_json::to_string(&status_installed)?;
    let deserialized_status: TilesetInstallationStatus =
      serde_json::from_str(&json_status)?;
    assert_eq!(status_installed, deserialized_status);

    let status_not_installed =
      TilesetInstallationStatus::NotInstalled;
    let json_status_not =
      serde_json::to_string(&status_not_installed)?;
    let deserialized_status_not: TilesetInstallationStatus =
      serde_json::from_str(&json_status_not)?;
    assert_eq!(status_not_installed, deserialized_status_not);

    Ok(())
  }
}
