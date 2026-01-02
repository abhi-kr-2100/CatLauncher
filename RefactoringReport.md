# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the standards defined in `AGENTS.md`. Each section details the issue and the proposed fix.

## Frontend

- **[Inconsistency 1]**: Incorrect Directory Structure for `AboutPage`.
  - **File(s)**: `cat-launcher/src/pages/AboutPage.tsx`, `cat-launcher/src/routes.tsx`
  - **Issue**: The `AboutPage` feature is a single file (`AboutPage.tsx`) directly in the `pages` directory. According to `AGENTS.md`, each feature should be a self-contained directory within `pages` (e.g., `pages/about/index.tsx`).
  - **Suggested Fix**: Create a new directory `cat-launcher/src/pages/AboutPage` and move the contents of `AboutPage.tsx` to a new `index.tsx` file inside it. Update the import path in `cat-launcher/src/routes.tsx`.
- **[Inconsistency 2]**: String Manipulation for UI Text.
  - **File(s)**: `cat-launcher/src/lib/utils.ts`
  - **Issue**: The `getHumanFriendlyText` function uses `replace` to format a string for display, which can cause issues with internationalization. `AGENTS.md` recommends avoiding string manipulation in favor of mapping functions or constants.
  - **Suggested Fix**: Replace the usage of `getHumanFriendlyText` with a mapping object or function that provides the correct display text for each input value. This will make the code more robust for localization.
- **[Inconsistency 3]**: String Construction for Timestamps.
  - **File(s)**: `cat-launcher/src/pages/BackupsPage/index.tsx`
  - **Issue**: The `formatTimestampForSearch` function manually constructs a date string for display. This is not localization-friendly, as different regions have different date and time formats.
  - **Suggested Fix**: Use a dedicated date and time formatting library that supports localization, such as `Intl.DateTimeFormat` or a third-party library like `date-fns` or `moment.js`. This will ensure that dates and times are displayed in a format appropriate for the user's locale.

## Backend

- **[Inconsistency 1]**: Incorrect Directory Structure in `play_time` feature.
  - **File(s)**: `cat-launcher/src-tauri/src/play_time/mod.rs`, `cat-launcher/src-tauri/src/play_time/repository.rs`, `cat-launcher/src-tauri/src/play_time/sqlite_play_time_repository.rs`
  - **Issue**: The `play_time` feature does not follow the prescribed directory structure. The `repository.rs` and `sqlite_play_time_repository.rs` files should be inside a `repository` subdirectory.
  - **Suggested Fix**: Create a `repository` directory within `cat-launcher/src-tauri/src/play_time` and move the `repository.rs` and `sqlite_play_time_repository.rs` files into it. Update `mod.rs` to reflect the new structure.
- **[Inconsistency 2]**: Incorrect Directory Structure in `theme` feature.
  - **File(s)**: `cat-launcher/src-tauri/src/theme/mod.rs`, `cat-launcher/src-tauri/src/theme/theme_preference_repository.rs`, `cat-launcher/src-tauri/src/theme/sqlite_theme_preference_repository.rs`
  - **Issue**: The `theme` feature does not follow the prescribed directory structure. The `theme_preference_repository.rs` and `sqlite_theme_preference_repository.rs` files should be inside a `repository` subdirectory.
  - **Suggested Fix**: Create a `repository` directory within `cat-launcher/src-tauri/src/theme` and move the `theme_preference_repository.rs` and `sqlite_theme_preference_repository.rs` files into it. Update `mod.rs` to reflect the new structure.
- **[Inconsistency 3]**: Incorrect Error Handling in `UsersRepository`.
  - **File(s)**: `cat-launcher/src-tauri/src/users/repository/users_repository.rs`
  - **Issue**: The `UsersRepositoryError` enum uses `Box<dyn std::error::Error + Send + Sync>` to wrap the underlying error. `AGENTS.md` recommends using `#[from]` to compose errors, which is more specific and avoids dynamic dispatch.
  - **Suggested Fix**: Refactor the `UsersRepositoryError` enum to use `#[from]` with a specific error type from the underlying database driver (e.g., `rusqlite::Error`). This will improve type safety and make the error handling more consistent with the rest of the codebase.
