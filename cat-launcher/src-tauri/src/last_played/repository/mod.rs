pub mod last_played_repository;
pub mod sqlite_last_played_repository;

pub use last_played_repository::{
    LastPlayedVersionRepository, LastPlayedVersionRepositoryError,
};
