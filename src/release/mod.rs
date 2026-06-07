pub(crate) const DEFAULT_TIMESTAMP: &str = "2024-01-01T00:00:00Z";

mod builder;
mod handler;
mod types;

pub use handler::{get_latest_release, get_release, get_release_by_tag, list_releases};
pub use types::Release;
