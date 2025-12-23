# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`. It also includes the suggested fixes for each inconsistency.

## Frontend

### Directory Structure

- **Inconsistency:** `cat-launcher/src/pages/AboutPage.tsx` is a file, but `AGENTS.md` specifies that features under `pages` should be directories.
- **Suggested Fix:** Convert `AboutPage.tsx` into a directory structure as specified in `AGENTS.md`. This involves creating a new directory `cat-launcher/src/pages/AboutPage` and moving the component to `cat-launcher/src/pages/AboutPage/index.tsx`.

- **Inconsistency:** The directory `cat-launcher/src/pages/game-tips` is not in PascalCase, which is inconsistent with the other page components.
- **Suggested Fix:** Rename the directory `cat-launcher/src/pages/game-tips` to `cat-launcher/src/pages/GameTipsPage`.

## Backend

### Directory Structure

- **Inconsistency:** The `backups` feature in `cat-launcher/src-tauri/src/backups` does not follow the directory structure specified in `AGENTS.md`. It is missing `lib.rs`, `types.rs`, and a `repository` directory. It also contains a non-standard `backups.rs` file.
- **Suggested Fix:** Refactor the `backups` feature to follow the specified directory structure. This involves creating the missing files and directories and moving the existing code from `backups.rs` into the appropriate files.

### Repository Ownership

- **Inconsistency:** The `backups` feature depends on the `launch_game` feature for its repository (`SqliteBackupRepository`). `AGENTS.md` specifies that each feature should be self-contained.
- **Suggested Fix:** Move the backup repository from `cat-launcher/src-tauri/src/launch_game/repository` to `cat-launcher/src-tauri/src/backups/repository`.
