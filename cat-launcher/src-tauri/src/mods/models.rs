use serde::Serialize;
use ts_rs::TS;

use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct StockMod {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub category: String,
    pub variant: GameVariant,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ThirdPartyMod {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub maintainers: Vec<String>,
    pub category: String,
    pub repository: String,
    pub variant: GameVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ThirdPartyModStatus>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ThirdPartyModStatus {
    pub variant: GameVariant,
    pub mod_id: String,
    pub installed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(tag = "source", content = "mod")]
#[ts(export)]
pub enum ModEntry {
    Stock(StockMod),
    ThirdParty(ThirdPartyMod),
}
