use std::io;
use std::path::Path;

use tokio::fs::{create_dir_all, read_dir};
use tokio::process::Command;

use crate::infra::utils::OS;

pub fn get_safe_filename(name: &str) -> String {
  name
    .chars()
    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
    .collect()
}

#[derive(thiserror::Error, Debug)]
pub enum CopyDirError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),
}

pub async fn copy_dir_all(
  src: &Path,
  dst: &Path,
  os: &OS,
) -> Result<(), CopyDirError> {
  create_dir_all(&dst).await?;

  // MacOS has many quirks. For example, attached DMGs can't be copied like
  // other directories. Use ditto to be safe.
  if os == &OS::Mac {
    let status =
      Command::new("ditto").arg(src).arg(dst).status().await?;
    if !status.success() {
      return Err(CopyDirError::Io(io::Error::other(
        "ditto command failed",
      )));
    }

    return Ok(());
  }

  let mut entries = read_dir(src).await?;
  while let Some(entry) = entries.next_entry().await? {
    let ty = entry.file_type().await?;
    if ty.is_dir() {
      Box::pin(copy_dir_all(
        &entry.path(),
        &dst.join(entry.file_name()),
        os,
      ))
      .await?;
    } else {
      tokio::fs::copy(&entry.path(), &dst.join(entry.file_name()))
        .await?;
    }
  }

  Ok(())
}
