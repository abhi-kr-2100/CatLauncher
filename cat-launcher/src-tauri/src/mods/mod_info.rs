use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ModType {
    Stock,
    ThirdParty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ModStatus {
    Installed,
    NotInstalled,
}

/// Shared mod information structure used for both stock and third-party mods.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

pub type ModsJson = std::collections::HashMap<String, Vec<ThirdPartyModEntry>>;

#[derive(Debug, Clone, Deserialize)]
pub struct ThirdPartyModEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub installation: InstallationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThirdPartyMod {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub installation: InstallationInfo,
}

impl From<ThirdPartyModEntry> for ThirdPartyMod {
    fn from(entry: ThirdPartyModEntry) -> Self {
        Self {
            id: entry.id,
            name: entry.name,
            description: entry.description,
            category: entry.category,
            installation: entry.installation,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InstallationInfo {
    pub download_url: String,
    pub mod_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Mod {
    pub mod_type: ModType,
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub status: ModStatus,
}

pub type ModInfoJsonEntry = ModInfo;

pub type ModInfoJson = Vec<ModInfoJsonEntry>;
