use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::variants::GameVariant;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub enum ReleaseType {
    Stable,
    Experimental,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct GameRelease {
    pub variant: GameVariant,
    pub version: String,
    pub release_type: ReleaseType,
}
