use std::collections::HashMap;
use std::num::{NonZeroU16, NonZeroUsize};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::basic_info::basic_info::Link;
use crate::constants::{DEFAULT_MAX_BACKUPS, DEFAULT_PARALLEL_REQUESTS};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub name: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub max_backups: NonZeroUsize,
    pub parallel_requests: NonZeroU16,
    pub games: HashMap<String, GameSettings>,
}

#[derive(Debug, Error)]
pub enum LoadSettingsError {
    #[error("Could not open settings.json")]
    OpenFile(#[source] std::io::Error),

    #[error("Could not parse settings.json")]
    Parse(#[from] serde_json::Error),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_backups: NonZeroUsize::new(DEFAULT_MAX_BACKUPS).unwrap(),
            parallel_requests: NonZeroU16::new(DEFAULT_PARALLEL_REQUESTS).unwrap(),
            games: HashMap::new(),
        }
    }
}
