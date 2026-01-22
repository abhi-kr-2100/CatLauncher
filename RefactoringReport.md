# Refactoring Report

This report details the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`.

## Frontend

### 1. Directory Structure

- **Issue**: The `cat-launcher/src/pages` directory does not follow the prescribed structure. `AboutPage.tsx` and `game-tips` are not in their own directories. `PlayPage` is missing a `components` subdirectory.
- **Fix**:
    - Create a directory for each page: `AboutPage` and `GameTipsPage`.
    - Move `AboutPage.tsx` to `cat-launcher/src/pages/AboutPage/index.tsx`.
    - Rename `game-tips` to `GameTipsPage`.
    - Create a `components` directory in `cat-launcher/src/pages/PlayPage` and move all component files into it.

### 2. Internationalization

- **Issue**: Hardcoded strings are used in `cat-launcher/src/pages/PlayPage/index.tsx`.
- **Fix**: Replace the hardcoded strings with a proper internationalization solution. Since there is no existing i18n library, I will create a simple mapping for the strings.

### 3. Custom Hooks

- **Issue**: The `useGameVariants` hook in `cat-launcher/src/hooks/useGameVariants.ts` does not use `useRef` for error callbacks, which can lead to stale closures.
- **Fix**: Refactor the hook to use `useRef` for the `onOrderUpdateError` and `onFetchError` callbacks, as suggested in `AGENTS.md`.

## Backend

### 1. Directory Structure

- **Issue**: In the `variants` feature, the file `game_variant.rs` should be named `types.rs` to follow the convention of storing types in a `types.rs` file.
- **Fix**: Rename `cat-launcher/src-tauri/src/variants/game_variant.rs` to `cat-launcher/src-tauri/src/variants/types.rs` and update the `mod.rs` file.
