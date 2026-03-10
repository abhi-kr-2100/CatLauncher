# Refactoring Report

This report documents inconsistencies found in the codebase compared to the coding standards defined in `AGENTS.md` and provides suggested fixes.

## Frontend Inconsistencies

### Data Fetching and Mutations

#### 1. Raw `useQuery` usage in providers
- **File**: `cat-launcher/src/providers/PostHogProviderWithIdentifiedUser.tsx`
- **Inconsistency**: Uses raw `useQuery` to fetch `userId`.
- **Suggested Fix**: Create a `useUserId` hook in `cat-launcher/src/hooks` that wraps `useQuery` and implements the required error handling.

#### 2. Missing custom hooks for some features
- **File**: `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts`
- **Inconsistency**: Returns `useQuery` directly without wrapping it in a custom hook that returns the necessary data/loading/error states along with ref-based error handling.
- **Suggested Fix**: Update `useGetTips` to follow the standard hook pattern with an optional `onGetTipsError` callback.

#### 3. Missing ref-based error handling in existing hooks
- **Files**:
  - `cat-launcher/src/hooks/useManualBackups.ts`
  - `cat-launcher/src/hooks/useBackups.ts`
  - `cat-launcher/src/pages/ModsPage/hooks/useGetLastModActivity.ts`
  - `cat-launcher/src/pages/ModsPage/hooks/useGetThirdPartyModInstallationStatus.ts`
- **Inconsistency**: These hooks do not implement the `useRef` for error callbacks as specified in `AGENTS.md`.
- **Suggested Fix**: Update these hooks to accept error callbacks and use `useRef` and `useEffect` to handle error reporting consistently.

#### 4. Hardcoded Query Keys
- **File**: `cat-launcher/src/pages/AchievementsPage/hooks/useAchievements.ts`
- **Inconsistency**: Uses a hardcoded query key `["achievements", null]` instead of only using `queryKeys.ts`.
- **Suggested Fix**: Move all query keys used in this hook to `cat-launcher/src/lib/queryKeys.ts`.

### Strings (Internationalization)

- **File**: Multiple `.tsx` files (e.g., `AboutPage.tsx`, `BackupsPage/index.tsx`, `PlayPage/index.tsx`, etc.).
- **Inconsistency**: Hardcoded strings like "Loading...", "Close", "Active", "Font", etc., are used directly in JSX.
- **Suggested Fix**: Move these strings to an i18n system or at least extract them to constants/translation files if an i18n system is available. (Note: Since the project's i18n system wasn't fully identified, at least mark them for future refactoring).

### Directory Structure

#### 1. Non-standard page folders
- **File**: `cat-launcher/src/pages/AboutPage.tsx`
- **Inconsistency**: `AboutPage` is a single file instead of a folder with `index.tsx`.
- **Suggested Fix**: Move `AboutPage.tsx` to `cat-launcher/src/pages/AboutPage/index.tsx`.

#### 2. Non-standard naming
- **File**: `cat-launcher/src/pages/game-tips`
- **Inconsistency**: Folder name is `game-tips` (kebab-case) instead of `GameTipsPage` (PascalCase) to match other features.
- **Suggested Fix**: Rename the folder to `GameTipsPage`.

#### 3. Hooks organization
- **Files**:
  - `cat-launcher/src/pages/SoundpacksPage/hooks.ts`
  - `cat-launcher/src/pages/TilesetsPage/hooks.ts`
- **Inconsistency**: Uses a `hooks.ts` file instead of a `hooks` directory within the feature folder.
- **Suggested Fix**: Move these to `hooks/index.ts` or individual hook files within a `hooks` folder.

## Backend Inconsistencies

### Commands and Business Logic

#### 1. Leakage of logic in commands
- **File**: `cat-launcher/src-tauri/src/install_release/commands.rs`
- **Inconsistency**: The command `install_release` performs multiple steps (getting OS, Arch, fetching release, then installing) instead of calling a single business logic function.
- **Suggested Fix**: Create a dedicated business logic function in `install_release/install_release.rs` (if not already present) that encapsulates these steps, and keep the command simple.

### Error Handling

#### 1. Command Error naming
- **Inconsistency**: Some commands use `{FunctionName}CommandError` while `AGENTS.md` says "{FunctionName}Error". However, the example in `AGENTS.md` uses `ActiveReleaseCommandError`.
- **Action**: Standardize on `{FunctionName}CommandError` for Tauri commands to distinguish them from business logic errors. (Currently mostly followed).

#### 2. Missing Error handling in `confirm_quit`
- **File**: `cat-launcher/src-tauri/src/lib.rs`
- **Inconsistency**: `confirm_quit` command doesn't return a `Result` and doesn't follow the error reporting pattern.
- **Suggested Fix**: Even if it just calls `exit`, it should ideally follow the pattern if any error could occur.

## Styling

- **Files**:
  - `cat-launcher/src/components/virtualized-combobox.tsx`
  - `cat-launcher/src/pages/SettingsPage/components/FontSettings.tsx`
- **Inconsistency**: Uses inline `style={{ ... }}` for things that could potentially be handled by Tailwind (except for highly dynamic values like progress bar widths).
- **Suggested Fix**: Replace manual styles with Tailwind classes where possible.
