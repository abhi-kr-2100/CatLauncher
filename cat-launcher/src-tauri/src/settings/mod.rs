use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub max_backups: NonZeroUsize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_backups: NonZeroUsize::new(5).unwrap(),
        }
    }
}
