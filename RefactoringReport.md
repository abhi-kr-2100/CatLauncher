# Refactoring Report

This report identifies inconsistencies in the CatLauncher codebase based on the standards defined in `AGENTS.md`.

## Frontend Inconsistencies

### 1. Direct use of `useQuery` in Components
- **File:** `cat-launcher/src/providers/PostHogProviderWithIdentifiedUser.tsx`
- **Issue:** The `useQuery` hook is used directly to fetch `userId`. `AGENTS.md` states that raw `useQuery` and `useMutation` hooks should not be used; instead, custom hooks should wrap them.
- **Suggested Fix:** Create a `useUserId` hook in `cat-launcher/src/hooks/useUserId.ts` and use it in the provider.

### 2. Custom Hooks Lacking Standard Error Callback Pattern
- **Files:**
    - `cat-launcher/src/hooks/useManualBackups.ts`
    - `cat-launcher/src/hooks/useBackups.ts`
    - `cat-launcher/src/hooks/useGameVariants.ts`
    - `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts`
    - `cat-launcher/src/pages/SettingsPage/hooks/useColorThemes.ts`
    - `cat-launcher/src/pages/SettingsPage/hooks/useSettingsForm.ts`
    - `cat-launcher/src/pages/SettingsPage/hooks/useFonts.ts`
    - `cat-launcher/src/pages/PlayPage/hooks/usePlayTime.ts`
    - `cat-launcher/src/pages/PlayPage/hooks/useReleaseNotes.ts`
    - `cat-launcher/src/pages/AchievementsPage/hooks/useAchievements.ts`
    - `cat-launcher/src/theme/useTheme.ts`
    - `cat-launcher/src/pages/TilesetsPage/hooks.ts`
    - `cat-launcher/src/pages/ModsPage/hooks/useGetThirdPartyModInstallationStatus.ts`
    - `cat-launcher/src/pages/ModsPage/hooks/useGetLastModActivity.ts`
    - `cat-launcher/src/pages/SoundpacksPage/hooks.ts`
- **Issue:** These hooks do not implement the `useRef` and `useEffect` pattern for error callbacks as specified in `AGENTS.md`.
- **Suggested Fix:** Refactor these hooks to accept optional error callbacks and use the standard pattern to trigger them.

### 3. Non-Standard Directory Structure
- **File:** `cat-launcher/src/pages/AboutPage.tsx`
- **Issue:** `AGENTS.md` specifies that each feature should be in its own directory with an `index.tsx` file.
- **Suggested Fix:** Move `AboutPage.tsx` to `cat-launcher/src/pages/AboutPage/index.tsx` and update relevant imports.

## Backend Inconsistencies

### 1. Command Error Naming Conventions
- **Files:**
    - `cat-launcher/src-tauri/src/game_tips/commands.rs` (`GetTipsCommandError`)
    - `cat-launcher/src-tauri/src/fetch_releases/commands.rs` (`FetchReleasesCommandError`, `FetchReleaseNotesCommandError`)
    - `cat-launcher/src-tauri/src/active_release/commands.rs` (`ActiveReleaseCommandError`)
- **Issue:** Error names for commands do not follow the `{FunctionName}Error` convention.
- **Suggested Fix:** Rename these error enums to match the command function names (e.g., `GetTipsError`, `FetchReleasesForVariantError`).

### 2. Potential Business Logic in Commands
- **File:** `cat-launcher/src-tauri/src/game_tips/commands.rs`
- **Issue:** The `get_tips` command performs OS enum detection and path resolution.
- **Suggested Fix:** While it prepares arguments, ensuring all logic is moved to the business logic layer where possible is preferred.
