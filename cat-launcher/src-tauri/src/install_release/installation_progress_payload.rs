use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InstallationProgressStatus {
    Downloading,
    Installing,
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InstallationProgressPayload {
    pub status: InstallationProgressStatus,
    pub release_id: String,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
}
