use std::fs::{create_dir_all, read_dir, File};
use std::io;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;
use tokio::task::JoinError;
use zip::result::ZipError;
use zip::write::FileOptions;
use zip::CompressionMethod::Deflated;
use zip::ZipWriter;

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

#[derive(thiserror::Error, Debug)]
pub enum ArchiveCreationError {
    #[error("destination is a directory")]
    DestinationIsDirectory,

    #[error("invalid or non-existent source directory")]
    InvalidOrNonExistentSourceDir,

    #[error("file IO operation failed: {0}")]
    Io(#[from] io::Error),

    #[error("failed to create archive file: {0}")]
    Failed(#[from] ZipError),

    #[error("failed to add to archive: {0}")]
    Add(#[from] AddToZipError),

    #[error("unexpected join error: {0}")]
    Join(#[from] JoinError),
}

pub async fn create_zip_archive(
    source_dir: &Path,
    paths_to_include: &[PathBuf],
    archive_path: &Path,
) -> Result<(), ArchiveCreationError> {
    let source_dir = source_dir.to_owned();
    let archive_path = archive_path.to_owned();

    if archive_path.is_dir() {
        return Err(ArchiveCreationError::DestinationIsDirectory);
    }

    if !source_dir.is_dir() {
        return Err(ArchiveCreationError::InvalidOrNonExistentSourceDir);
    }

    let paths_to_include: Vec<PathBuf> = paths_to_include.iter().map(|p| p.to_path_buf()).collect();

    tokio::task::spawn_blocking(move || {
        let file = File::create(&archive_path)?;
        let mut zip = ZipWriter::new(file);

        for path_to_add in &paths_to_include {
            add_to_zip(&mut zip, &source_dir, path_to_add)?;
        }

        zip.finish()?;

        Ok(())
    })
    .await?
}

#[derive(thiserror::Error, Debug)]
pub enum AddToZipError {
    #[error("failed to add to zip file: {0}")]
    Failed(#[from] ZipError),

    #[error("file IO operation failed: {0}")]
    Io(#[from] io::Error),

    #[error("path is not inside base directory")]
    InvalidPath,
}

fn add_to_zip(
    zip: &mut ZipWriter<File>,
    base_path: &Path,
    path_to_add: &PathBuf,
) -> Result<(), AddToZipError> {
    let relative_path = path_to_add
        .strip_prefix(base_path)
        .map_err(|_| AddToZipError::InvalidPath)?;

    let options: FileOptions<'_, ()> = FileOptions::default().compression_method(Deflated);

    if path_to_add.is_file() {
        zip.start_file(relative_path.to_string_lossy(), options)?;
        let mut file = File::open(&path_to_add)?;
        io::copy(&mut file, zip)?;
    } else if path_to_add.is_dir() {
        if !relative_path.as_os_str().is_empty() {
            let dir_path_str = format!("{}/", relative_path.to_string_lossy());
            zip.add_directory(&dir_path_str, options)?;
        }

        for entry in read_dir(&path_to_add)? {
            let entry = entry?;
            add_to_zip(zip, base_path, &entry.path())?;
        }
    }

    Ok(())
}
