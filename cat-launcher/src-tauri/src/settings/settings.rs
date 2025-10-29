use std::collections::HashMap;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::basic_info::basic_info::Link;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub name: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub max_backups: NonZeroUsize,
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
            max_backups: NonZeroUsize::new(5).unwrap(),
            games: HashMap::new(),
        }
    }
}
