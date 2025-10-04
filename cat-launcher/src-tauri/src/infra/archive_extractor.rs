use std::{
    fs::{create_dir_all, File},
    io,
    path::Path,
};

use flate2::read::GzDecoder;
use tar::Archive;
use tokio::task::JoinError;
use zip::result::ZipError;

#[derive(thiserror::Error, Debug)]
pub enum ExtractionError {
    #[error("unsupported archive format")]
    UnsupportedArchive,

    #[error("extraction failed: {0}")]
    Extraction(#[from] io::Error),

    #[error("zip extraction failed: {0}")]
    Zip(#[from] ZipError),

    #[error("unexpected join error: {0}")]
    Join(#[from] JoinError),
}

pub async fn extract_archive(
    archive_path: &Path,
    target_dir: &Path,
) -> Result<(), ExtractionError> {
    let archive_path = archive_path.to_owned();
    let target_dir = target_dir.to_owned();

    tokio::task::spawn_blocking(move || {
        let extension = archive_path.extension().and_then(|s| s.to_str());

        let file_stem_extension = archive_path
            .file_stem()
            .and_then(|s| Path::new(s).extension())
            .and_then(|s| s.to_str());

        if !target_dir.exists() {
            create_dir_all(&target_dir)?;
        }

        match extension {
            Some("zip") => {
                let file = File::open(&archive_path)?;
                let mut archive = zip::ZipArchive::new(file)?;
                archive.extract(&target_dir)?;
            }

            Some("gz") => match file_stem_extension {
                Some("tar") => {
                    let file = File::open(&archive_path)?;
                    let tar = GzDecoder::new(file);
                    let mut archive = Archive::new(tar);
                    archive.unpack(&target_dir)?;
                }
                _ => return Err(ExtractionError::UnsupportedArchive),
            },

            _ => return Err(ExtractionError::UnsupportedArchive),
        }

        Ok(())
    })
    .await?
}
