// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2020 Tobias Hunger <tobias.hunger@gmail.com>

//! The `Downloader` that holds all the logic to manage the `Downloads`

use crate::{Download, DownloadSummary, Error, Result};

use crate::progress::Factory;

// ----------------------------------------------------------------------
// - Helper:
// ----------------------------------------------------------------------

fn validate_downloads(
    downloads: &[Download],
    download_folder: &std::path::Path,
    factory: &dyn Factory,
) -> Result<Vec<Download>> {
    let mut known_urls = std::collections::HashSet::new();
    let mut known_download_paths = std::collections::HashSet::new();

    let mut result = Vec::with_capacity(downloads.len());

    for d in downloads {
        if d.urls.is_empty() {
            return Err(Error::DownloadDefinition(String::from(
                "No URL found to download.",
            )));
        }

        for u in &d.urls {
            if !known_urls.insert(u) {
                return Err(Error::DownloadDefinition(format!(
                    "Download URL \"{u}\" is used more than once.",
                )));
            }
        }

        let urls = d.urls.clone();

        if d.file_name.to_string_lossy().is_empty() {
            return Err(Error::DownloadDefinition(String::from(
                "No download file name was provided.",
            )));
        }

        let file_name = download_folder.join(&d.file_name);
        if d.file_name.to_string_lossy().is_empty() {
            return Err(Error::DownloadDefinition(String::from(
                "Failed to get full download path.",
            )));
        }

        if !known_download_paths.insert(&d.file_name) {
            return Err(Error::DownloadDefinition(format!(
                "Download file name \"{}\" is used more than once.",
                d.file_name.to_string_lossy(),
            )));
        }

        let progress = if d.progress.is_none() {
            factory.create_reporter()
        } else {
            d.progress.as_ref().expect("Was Some just now...").clone()
        };

        result.push(Download {
            urls,
            file_name,
            progress: Some(progress),
        });
    }

    Ok(result)
}

// ----------------------------------------------------------------------
// - Downloader:
// ----------------------------------------------------------------------

/// This is the main entry point: You need to have a `Downloader` and then can call
/// `download` on that, passing in a list of `Download` objects.
pub struct Downloader<C> {
    client: C,
    parallel_requests: u16,
    retries: u16,
    download_folder: std::path::PathBuf,
}

impl<C: crate::HttpClient> Downloader<C> {
    /// Start the download
    ///
    /// # Errors
    /// `Error::DownloadDefinition` if the download is detected to be broken in some way.
    pub fn download(&mut self, downloads: &[Download]) -> Result<Vec<Result<DownloadSummary>>> {
        let factory = crate::progress::Noop::default();

        let to_process = validate_downloads(downloads, &self.download_folder, &factory)?;
        if to_process.is_empty() {
            return Ok(Vec::new());
        }

        Ok(crate::backend::run(
            &mut self.client,
            to_process,
            self.retries,
            self.parallel_requests,
        ))
    }

    /// Start the download asyncroniously
    ///
    /// # Errors
    /// `Error::DownloadDefinition` if the download is detected to be broken in some way.
    pub async fn async_download(
        &mut self,
        downloads: &[Download],
    ) -> Result<Vec<Result<DownloadSummary>>> {
        let factory = crate::progress::Noop::default();

        let to_process = validate_downloads(downloads, &self.download_folder, &factory)?;
        if to_process.is_empty() {
            return Ok(Vec::new());
        }

        let result = crate::backend::async_run(
            &mut self.client,
            to_process,
            self.retries,
            self.parallel_requests,
        )
        .await;

        Ok(result)
    }
}

// ----------------------------------------------------------------------
// - Builder:
// ----------------------------------------------------------------------

/// A builder for a `Downloader`
pub struct Builder {
    parallel_requests: u16,
    retries: u16,
    download_folder: std::path::PathBuf,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            parallel_requests: 32,
            retries: 3,
            download_folder: std::path::PathBuf::from("."),
        }
    }
}

impl Builder {
    /// Set the number of parallel requests.
    ///
    /// The default is 32.
    pub fn parallel_requests(&mut self, count: u16) -> &mut Self {
        self.parallel_requests = count;
        self
    }

    /// Set the number of retries.
    ///
    /// The default is 3.
    pub fn retries(&mut self, count: u16) -> &mut Self {
        self.retries = count;
        self
    }

    /// Set the folder to download into.
    ///
    /// The default is unset and a value is required.
    pub fn download_folder(&mut self, folder: &std::path::Path) -> &mut Self {
        self.download_folder = folder.to_path_buf();
        self
    }

    /// Build a downloader with a specified client.
    ///
    /// # Errors
    /// * `Error::Setup`, when setup fails
    pub fn build_with_client<C: crate::HttpClient>(&mut self, client: C) -> crate::Result<Downloader<C>> {
        let download_folder = &self.download_folder;
        if download_folder.to_string_lossy().is_empty() {
            return Err(crate::Error::Setup(
                "Required \"download_folder\" was not set.".into(),
            ));
        }
        if !download_folder.is_dir() {
            return Err(Error::Setup(format!(
                "Required \"download_folder\" with value \"{}\" is not a folder.",
                download_folder.to_string_lossy()
            )));
        }

        Ok(Downloader {
            client,
            parallel_requests: self.parallel_requests,
            retries: self.retries,
            download_folder: download_folder.clone(),
        })
    }
}

