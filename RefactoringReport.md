# Refactoring Report

This report details the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`.

## Frontend

### 1. Incorrect Directory Structure in `PlayPage`

- **Inconsistency:** The `PlayPage` feature located at `cat-launcher/src/pages/PlayPage` does not follow the prescribed directory structure. Components and hooks are not in their respective `components` and `hooks` directories.
- **Suggested Fix:**
    - Create a `components` directory inside `cat-launcher/src/pages/PlayPage` and move `GameVariantCard.tsx`, `InteractionButton.tsx`, `PlayTime.tsx`, `ReleaseFilter.tsx`, `ReleaseLabel.tsx`, and `ReleaseSelector.tsx` into it.
    - Create a `hooks` directory inside `cat-launcher/src/pages/PlayPage` and move `hooks.ts` into it, splitting the hooks into individual files.

### 2. Raw `useQuery` and `useMutation` Usage

- **Inconsistency:** The `cat-launcher/src/pages/PlayPage/hooks.ts` file uses raw `useQuery` and `useMutation` hooks from `tanstack-query`. The `AGENTS.md` file states that custom hooks should be created to wrap these.
- **Suggested Fix:**
    - Refactor the hooks in `cat-launcher/src/pages/PlayPage/hooks.ts` to use custom hooks that wrap `useQuery` and `useMutation`.

### 3. Multiple Hooks in a Single File

- **Inconsistency:** The `cat-launcher/src/pages/PlayPage/hooks.ts` file contains multiple hooks.
- **Suggested Fix:**
    - Split the hooks into individual files within the `cat-launcher/src/pages/PlayPage/hooks` directory.

## Backend

### 1. Incorrect Error Handling in `sqlite_active_release_repository.rs`

- **Inconsistency:** The `cat-launcher/src-tauri/src/active_release/repository/sqlite_active_release_repository.rs` file uses a generic `ActiveReleaseRepositoryError` enum with `Get` and `Set` variants. The `AGENTS.md` file specifies that each function should have its own error enum.
- **Suggested Fix:**
    - Create `GetActiveReleaseError` and `SetActiveReleaseError` enums in `sqlite_active_release_repository.rs`.
    - Update the `get_active_release` and `set_active_release` functions to use these new error enums.
    - Update the `ActiveReleaseRepository` trait to use the new error enums.
