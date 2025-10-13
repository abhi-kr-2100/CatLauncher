use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Clone, TS)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum UpdateStatus {
    Checking,
    Downloading,
    Installing,
    UpToDate,
    Success,
    Failure { error: String },
}
