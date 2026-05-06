//! Repository module for game launch-related data persistence.

/// Repository traits and types for managing backups.
pub mod backup_repository;
/// SQLite implementation of the backup repository.
pub mod sqlite_backup_repository;

pub use backup_repository::{
  BackupEntry, BackupRepository, BackupRepositoryError,
};
