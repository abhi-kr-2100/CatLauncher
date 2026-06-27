// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2020 Tobias Hunger <tobias.hunger@gmail.com>

//! The `Download` struct is used to describe a file that is
//! supposed to get downloaded.

// ----------------------------------------------------------------------
// - Download:
// ----------------------------------------------------------------------

/// A `Download`.
pub struct Download {
    /// A list of URLs that this file can be retrieved from. `downloader` will pick
    /// the download URL from this list at random.
    pub urls: Vec<String>,
    /// A progress `Reporter` to report the download process with.
    pub progress: Option<crate::Progress>,
    /// The file name to be used for the downloaded file.
    pub file_name: std::path::PathBuf,
}

fn file_name_from_url(url: &str) -> std::path::PathBuf {
    if url.is_empty() {
        return std::path::PathBuf::new();
    }
    let Ok(url) = url::Url::parse(url) else {
        return std::path::PathBuf::new();
    };

    url.path_segments()
        .map_or_else(std::path::PathBuf::new, |f| {
            std::path::PathBuf::from(f.last().unwrap_or(""))
        })
}

impl Download {
    /// Create a new `Download` with a single download `url`
    #[must_use]
    pub fn new(url: &str) -> Self {
        Self {
            urls: vec![url.to_owned()],
            progress: None,
            file_name: file_name_from_url(url),
        }
    }

    /// Create a new `Download` based on a list of mirror urls.
    #[must_use]
    pub fn new_mirrored(urls: &[&str]) -> Self {
        let urls: Vec<String> = urls.iter().map(|s| String::from(*s)).collect();
        let url = urls.first().unwrap_or(&String::new()).clone();

        Self {
            urls,
            progress: None,
            file_name: file_name_from_url(&url),
        }
    }

    /// Set the name of the downloaded file. This filename can be absolute or
    /// relative to the `download_folder` defined in the `Downloader`.
    ///
    /// Default is the file name on the server side (if available)
    #[must_use]
    pub fn file_name(mut self, path: &std::path::Path) -> Self {
        self.file_name = path.to_path_buf();
        self
    }

    /// Register handling of progress information
    ///
    /// Defaults to not printing any progress information.
    #[must_use]
    pub fn progress(mut self, progress: crate::Progress) -> Self {
        self.progress = Some(progress);
        self
    }
}
