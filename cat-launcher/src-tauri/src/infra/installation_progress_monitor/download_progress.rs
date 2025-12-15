use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DownloadProgress {
  pub bytes_downloaded: u64,
  pub total_bytes: u64,
}
