use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InstallationProgressStatus {
    Downloading,
    Installing,
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InstallationProgressPayload {
    pub status: InstallationProgressStatus,
    pub release_id: String,
}
