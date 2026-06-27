// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2020 Tobias Hunger <tobias.hunger@gmail.com>

//! Progress reporting code

// ----------------------------------------------------------------------
// - Types:
// ----------------------------------------------------------------------

/// A Progress reporter to use for the `Download`
pub type Progress = std::sync::Arc<dyn Reporter>;

// ----------------------------------------------------------------------
// - Traits:
// ----------------------------------------------------------------------

/// An interface for `ProgressReporter`s
pub trait Reporter: Send + Sync {
    /// Setup a TUI element for the next progress
    fn setup(&self, max_progress: Option<u64>, message: &str);
    /// Report progress
    fn progress(&self, current: u64);
    /// Report progress
    fn set_message(&self, message: &str);
    /// Finish up after progress reporting is done
    fn done(&self);
}

/// A `Factory` used to create `Reporter` when a Download does
/// not come with a defined way to report progress already.
pub trait Factory {
    /// Create an `Reporter`
    fn create_reporter(&self) -> crate::Progress;
}

// ----------------------------------------------------------------------
// - Noop:
// ----------------------------------------------------------------------

/// Do not print anything
#[derive(Default)]
pub struct Noop {}

impl Reporter for Noop {
    fn setup(&self, _: Option<u64>, _: &str) {}

    fn progress(&self, _: u64) {}

    fn set_message(&self, _: &str) {}

    fn done(&self) {}
}

impl Noop {
    /// Create a `Noop` `Reporter`.
    #[must_use]
    pub fn create() -> crate::Progress {
        std::sync::Arc::new(Self {})
    }
}

impl Factory for Noop {
    fn create_reporter(&self) -> crate::Progress {
        Self::create()
    }
}
