# Refactoring Report

This report details the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`.

## Frontend

### `cat-launcher/src/hooks/useBackups.ts`

*   **Inconsistency**: The `useBackups` hook uses `useQuery` directly without the recommended error handling pattern.
*   **Suggested Fix**: Refactor the hook to use the `useRef` and `useEffect` pattern for the `onError` callback as described in `AGENTS.md`.

### `cat-launcher/src/hooks/useGameVariants.ts`

*   **Inconsistency**: The `useGameVariants` hook has an `onFetchError` callback but doesn't use the `useRef` and `useEffect` pattern.
*   **Suggested Fix**: Refactor the hook to use the `useRef` and `useEffect` pattern for the `onFetchError` callback.

### `cat-launcher/src/hooks/useManualBackups.ts`

*   **Inconsistency**: The `useManualBackups` hook uses `useQuery` directly without any error handling.
*   **Suggested Fix**: Add error handling to the hook using the `useRef` and `useEffect` pattern for the `onError` callback.

## Backend

### `cat-launcher/src-tauri/src/active_release/commands.rs`

*   **Inconsistency**: The `get_active_release` command passes a `State` object to the business logic function.
*   **Suggested Fix**: Extract the inner repository from the `State` object in the command and pass it to the business logic function.

### `cat-launcher/src-tauri/src/active_release/active_release.rs`

*   **Inconsistency**: The `ActiveReleaseError` enum is used for multiple functions (`get_active_release` and `set_active_release`). `AGENTS.md` requires one error enum per function, named `{FunctionName}Error`.
*   **Suggested Fix**: Create separate error enums for `get_active_release` and `set_active_release` functions, named `GetActiveReleaseError` and `SetActiveReleaseError` respectively.

### `cat-launcher/src-tauri/src/backups/commands.rs`

*   **Inconsistency**: The `list_backups_for_variant`, `delete_backup_by_id`, and `restore_backup_by_id` commands pass `State` and `AppHandle` objects to the business logic functions.
*   **Suggested Fix**: Extract the necessary data from the `State` and `AppHandle` objects in the commands and pass them to the business logic functions.

### `cat-launcher/src-tauri/src/backups/backups.rs`

*   **Inconsistency**: The error enums are not following the `{FunctionName}Error` convention and the function names are not consistent with the commands.
*   **Suggested Fix**: Rename `list_backups` to `list_backups_for_variant` and its error to `ListBackupsForVariantError`. Rename `delete_backup` to `delete_backup_by_id` and its error to `DeleteBackupByIdError`. Rename `restore_backup` to `restore_backup_by_id` and its error to `RestoreBackupByIdError`.
