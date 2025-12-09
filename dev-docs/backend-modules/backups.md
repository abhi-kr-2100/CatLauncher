# Backups Module

## Motivation

The `backups` module is responsible for creating and restoring backups of the game's save files. This is a critical feature for users who want to protect their progress, migrate saves, or recover from data corruption. The module handles the lifecycle of backups, including listing, deleting, and restoring them.

## Design

The module follows the project's vertical slice and clean architecture principles.

1.  **Core Business Logic (`backups.rs`):** This file contains the framework-agnostic logic for managing backups.
    *   `list_backups`: Fetches a sorted list of backup entries for a given game variant from the database.
    *   `delete_backup`: Deletes a backup entry from the database and its corresponding archive file from the disk. It includes a transactional safety measure: if the file deletion fails, the database entry is restored to prevent an orphaned file.
    *   `restore_backup`: Restores a selected backup. It finds the backup archive and extracts its contents to the appropriate user data directory, overwriting the current save files.

2.  **Framework Bridge (`commands.rs`):** This file exposes the core logic to the frontend via Tauri commands.
    *   `list_backups_for_variant`: A command that returns all backups for a specific game variant.
    *   `delete_backup_by_id`: A command that deletes a backup given its unique ID.
    *   `restore_backup_by_id`: A command that restores a backup given its unique ID.

3.  **Dependencies:**
    *   **`BackupRepository`:** The module depends on a `BackupRepository` trait (implemented by `SqliteBackupRepository`) to abstract the database interactions for storing backup metadata.
    *   **`filesystem` Module:** It uses helpers from the `filesystem` module to construct the correct paths for backup archives and user data directories.
    *   **`infra` Module:** It uses the `archive` utility from the `infra` module to handle the extraction of backup archives during the restore process.

## Workings

The backup process is tied to the game launch sequence. When a game is launched, the system automatically creates a compressed archive of the game's save directory and stores it in the application's data folder. A corresponding entry is created in the `backups` table in the SQLite database, storing metadata such as the timestamp, game variant, and release version.

When a user requests to restore a backup, the module finds the relevant archive file using the database metadata and extracts it to the active save game directory, allowing the user to revert their progress to that point in time.
