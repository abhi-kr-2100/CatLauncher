use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::variants::GameVariant;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct StockSoundpack {
    pub name: String,
    pub view: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ThirdPartySoundpack {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub internal_path: String,
    pub status: SoundpackInstallStatus,
    #[ts(type = "string | null")]
    pub last_updated_time: Option<chrono::DateTime<chrono::Utc>>,
    #[ts(type = "string | null")]
    pub installed_last_updated_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum Soundpack {
    #[serde(rename = "stock")]
    Stock(StockSoundpack),
    #[serde(rename = "third-party")]
    ThirdParty(ThirdPartySoundpack),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SoundpacksForVariant {
    pub variant: GameVariant,
    pub soundpacks: Vec<Soundpack>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum SoundpackInstallStatus {
    Installed,
    NotInstalled,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum SoundpackInstallProgressStatus {
    Downloading,
    Installing,
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SoundpackInstallStatusPayload {
    pub status: SoundpackInstallProgressStatus,
    pub variant: GameVariant,
    pub soundpack_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoundpackCatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub internal_path: String,
}

#[derive(Debug, Clone)]
pub struct InstalledSoundpackMetadata {
    pub soundpack_id: String,
    pub variant: GameVariant,
    pub installed_last_updated_time: chrono::DateTime<chrono::Utc>,
}
