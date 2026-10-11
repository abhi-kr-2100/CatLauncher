use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::infra::utils::Asset;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SoundpackInstallation {
  pub download_url: String,

  pub soundpack: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SoundpackActivity {
  pub activity_type: String,

  pub github: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThirdPartySoundpack {
  pub id: String,

  pub name: String,

  pub installation: SoundpackInstallation,

  pub activity: SoundpackActivity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StockSoundpack {
  pub id: String,

  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SoundpackInstallationStatus {
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
  fn test_soundpack_asset_trait_stock() {
    let stock = StockSoundpack {
      id: "cc-sounds".to_string(),
      name: "CC Sounds".to_string(),
    };
    let soundpack = Soundpack::Stock(stock);

    assert!(!soundpack.is_third_party());
    assert_eq!(soundpack.id(), "cc-sounds");
  }

  #[test]
  fn test_soundpack_asset_trait_third_party() {
    let tp = ThirdPartySoundpack {
      id: "oistein-soundpack".to_string(),
      name: "Oistein's Soundpack".to_string(),
      installation: SoundpackInstallation {
        download_url: "https://example.com/soundpack.zip".to_string(),
        soundpack: "soundpack.txt".to_string(),
      },
      activity: SoundpackActivity {
        activity_type: "github_commit".to_string(),
        github: Some("owner/repo".to_string()),
      },
    };
    let soundpack = Soundpack::ThirdParty(tp);

    assert!(soundpack.is_third_party());
    assert_eq!(soundpack.id(), "oistein-soundpack");
  }

  #[test]
  fn test_soundpack_serde_roundtrip() -> TestResult {
    let stock = Soundpack::Stock(StockSoundpack {
      id: "basic".to_string(),
      name: "Basic Soundpack".to_string(),
    });

    let json = serde_json::to_string(&stock)?;
    let deserialized: Soundpack = serde_json::from_str(&json)?;
    assert_eq!(stock, deserialized);

    let tp = Soundpack::ThirdParty(ThirdPartySoundpack {
      id: "tp-sound".to_string(),
      name: "Third Party Sound".to_string(),
      installation: SoundpackInstallation {
        download_url: "https://example.com/tp.zip".to_string(),
        soundpack: "data/soundpack.txt".to_string(),
      },
      activity: SoundpackActivity {
        activity_type: "github_release".to_string(),
        github: None,
      },
    });

    let json_tp = serde_json::to_string(&tp)?;
    let deserialized_tp: Soundpack = serde_json::from_str(&json_tp)?;
    assert_eq!(tp, deserialized_tp);

    let status_installed = SoundpackInstallationStatus::Installed;
    let json_status = serde_json::to_string(&status_installed)?;
    let deserialized_status: SoundpackInstallationStatus =
      serde_json::from_str(&json_status)?;
    assert_eq!(status_installed, deserialized_status);

    let status_not_installed =
      SoundpackInstallationStatus::NotInstalled;
    let json_status_not =
      serde_json::to_string(&status_not_installed)?;
    let deserialized_status_not: SoundpackInstallationStatus =
      serde_json::from_str(&json_status_not)?;
    assert_eq!(status_not_installed, deserialized_status_not);

    Ok(())
  }
}
