use std::io;
use std::path::Path;

use tokio::fs::{create_dir_all, read_dir};

pub fn get_safe_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

#[derive(thiserror::Error, Debug)]
pub enum CopyDirError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub async fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), CopyDirError> {
    create_dir_all(&dst).await?;

    let mut entries = read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;
        if ty.is_dir() {
            Box::pin(copy_dir_all(&entry.path(), &dst.join(entry.file_name()))).await?;
        } else {
            tokio::fs::copy(&entry.path(), &dst.join(entry.file_name())).await?;
        }
    }

    Ok(())
}
