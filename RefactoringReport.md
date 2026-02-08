# Refactoring Report

This report details the inconsistencies found in the codebase when compared against the coding standards outlined in `AGENTS.md`. It also includes the proposed fixes to bring the code into compliance.

## Frontend

### 1. `AboutPage.tsx` is a file, not a directory

- **Inconsistency**: `cat-launcher/src/pages/AboutPage.tsx` is a file, but `AGENTS.md` states that each feature page should be a directory.
- **Fix**: Convert `AboutPage.tsx` into a directory structure as `cat-launcher/src/pages/AboutPage/index.tsx`.

### 2. `game-tips` directory is not in PascalCase

- **Inconsistency**: The `cat-launcher/src/pages/game-tips` directory is not in PascalCase, which violates the naming convention for feature directories.
- **Fix**: Rename the directory to `cat-launcher/src/pages/GameTipsPage`.

### 3. `game-tips` directory is missing `index.tsx`

- **Inconsistency**: The `cat-launcher/src/pages/game-tips` directory is missing the main `index.tsx` file.
- **Fix**: Since `TipOfTheDay.tsx` is a component and not a page, an `index.tsx` file will be created to serve as the main page for the `GameTipsPage` feature.

### 4. `TipOfTheDay.tsx` is a component, not a page

- **Inconsistency**: `TipOfTheDay.tsx` is a component and is located in the root of the `game-tips` directory, which is not the correct place for it.
- **Fix**: Move `TipOfTheDay.tsx` to the `cat-launcher/src/pages/GameTipsPage/components` directory.

### 5. `useGetTips.ts` uses a raw `useQuery`

- **Inconsistency**: The `useGetTips` hook in `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts` uses a raw `useQuery`, which violates the `tanstack-query` guidelines in `AGENTS.md`.
- **Fix**: Refactor the hook to wrap `useQuery` and include error handling callbacks.

## Backend

### 1. `game_tips` directory is missing the `repository` subdirectory

- **Inconsistency**: The `cat-launcher/src-tauri/src/game_tips` directory is missing the `repository` subdirectory, which is required by the backend directory structure.
- **Fix**: Create the `repository` subdirectory and move the repository-related logic into it.

### 2. `get_tips` command in `commands.rs` uses `AppHandle`

- **Inconsistency**: The `get_tips` command in `cat-launcher/src-tauri/src/game_tips/commands.rs` uses the Tauri `AppHandle` directly, which is discouraged.
- **Fix**: Refactor the command to avoid using `AppHandle` and instead pass the required data directly to the business logic function.

### 3. `get_tips` command passes concrete SQLite implementations

- **Inconsistency**: The `get_tips` command passes concrete SQLite repository implementations to the business logic function, which violates the repository abstraction principle.
- **Fix**: Refactor the command to use the repository traits instead of the concrete implementations.
