# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared to the standards defined in `AGENTS.md`.

## Frontend

### Data Fetching and Mutations

- **Raw `useQuery` and `useMutation` hooks should be wrapped in custom hooks.**
  - **Inconsistency:** Found direct usage of `useQuery` in `cat-launcher/src/pages/PlayPage/hooks/useGameReleases.ts`.
  - **Suggested Fix:** Refactor to a custom hook that encapsulates the `useQuery` logic and handles errors as described in `AGENTS.md`.

- **All query keys must be stored in the `cat-launcher/src/lib/queryKeys.ts` file.**
  - **Inconsistency:** The query key `["releases", variant]` is used directly in `cat-launcher/src/pages/PlayPage/hooks/useGameReleases.ts`.
  - **Suggested Fix:** Move the query key to `cat-launcher/src/lib/queryKeys.ts` and use it from there.

- **Communication with the Tauri backend must only happen through the functions defined in the `cat-launcher/src/lib/commands.ts` file.**
  - **Inconsistency:** The `invoke` function from `@tauri-apps/api` is used directly in `cat-launcher/src/pages/PlayPage/hooks/useGameReleases.ts` to call the `get_releases` command.
  - **Suggested Fix:** Create a function in `cat-launcher/src/lib/commands.ts` that calls the `get_releases` command and use that function in the hook.

### Strings

- **All strings displayed to the user should be internationalization-ready.**
  - **Inconsistency:** Found hardcoded strings in `cat-launcher/src/pages/PlayPage/GameVariantCard.tsx`. For example, "Play" and "Install".
  - **Suggested Fix:** Use a translation library (like `i18next`) and replace hardcoded strings with translation keys.

- **Avoid string construction in parts.**
  - **Inconsistency:** In `cat-launcher/src/pages/PlayPage/PlayTime.tsx`, a string is constructed with `Played for ${hours}h ${minutes}m`.
  - **Suggested Fix:** Use a formatting function that can handle different locales and word orders.

### Directory Structure

- The directory structure seems to be mostly compliant.

## Backend

### Commands

- **Commands should be straightforward and should not perform business logic.**
  - **Inconsistency:** The `get_releases` command in `cat-launcher/src-tauri/src/fetch_releases/commands.rs` contains business logic for fetching releases.
  - **Suggested Fix:** Move the business logic to a separate function and have the command call that function.

### Error Handling

- **Define one error enum for every function. The error name should be `{FunctionName}Error`.**
  - **Inconsistency:** In `cat-launcher/src-tauri/src/fetch_releases/fetch_releases.rs`, the `fetch_releases` function returns a `anyhow::Result`, not a custom error enum.
  - **Suggested Fix:** Create a `FetchReleasesError` enum and use it as the return type for the `fetch_releases` function.

### Repository

- The repository pattern seems to be mostly compliant.
