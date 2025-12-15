use std::sync::atomic::{AtomicU64, Ordering};

use downloader::progress::Reporter;
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::infra::installation_progress_monitor::download_progress::DownloadProgress;

pub struct ChannelReporter {
  channel: Channel,
  // Use AtomicU64 for interior mutability across threads without locking.
  total_bytes: AtomicU64,
}

impl ChannelReporter {
  pub fn new(channel: Channel) -> Self {
    Self {
      channel,
      total_bytes: AtomicU64::new(0),
    }
  }
}

impl Reporter for ChannelReporter {
  fn setup(&self, max_progress: Option<u64>, _message: &str) {
    if let Some(total) = max_progress {
      self.total_bytes.store(total, Ordering::Relaxed);
    }
  }

  fn progress(&self, current: u64) {
    let total_bytes = self.total_bytes.load(Ordering::Relaxed);
    let progress = DownloadProgress {
      bytes_downloaded: current,
      total_bytes,
    };
    if let Ok(value) = serde_json::to_value(progress) {
      let _ = self
        .channel
        .send(InvokeResponseBody::Json(value.to_string()));
    }
  }

  fn set_message(&self, _message: &str) {}

  fn done(&self) {}
}
