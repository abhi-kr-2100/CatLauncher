use std::num::{NonZeroU16, NonZeroUsize};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::constants::{DEFAULT_MAX_BACKUPS, DEFAULT_PARALLEL_REQUESTS};

#[derive(Debug, Serialize, Deserialize, Clone, TS, sqlx::FromRow)]
#[ts(export)]
pub struct Settings {
    #[ts(type = "number")]
    pub max_backups: NonZeroUsize,
    #[ts(type = "number")]
    pub parallel_requests: NonZeroU16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_backups: NonZeroUsize::new(DEFAULT_MAX_BACKUPS).unwrap(),
            parallel_requests: NonZeroU16::new(DEFAULT_PARALLEL_REQUESTS).unwrap(),
        }
    }
}
