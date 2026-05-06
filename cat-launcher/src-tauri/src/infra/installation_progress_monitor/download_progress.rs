use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
/// Represents the progress of a file download.
pub struct DownloadProgress {
  /// The number of bytes already downloaded.
  pub bytes_downloaded: u64,
  /// The total number of bytes to be downloaded.
  pub total_bytes: u64,
}
