# Refactoring Report

This report documents the inconsistencies found in the codebase compared to the coding standards and advice in `AGENTS.md`.

## Frontend

### 1. Data Fetching and Mutations (Custom Hook Wrapper Pattern)
**Inconsistency**: Several custom hooks do not follow the mandatory `useRef` + `useEffect` pattern for error callbacks.
**Files**:
- `cat-launcher/src/hooks/useGameVariants.ts`
- `cat-launcher/src/pages/PlayPage/hooks/useInstallationStatus.ts`
- `cat-launcher/src/hooks/useBackups.ts`
- `cat-launcher/src/pages/ModsPage/hooks/useMods.ts`
- `cat-launcher/src/pages/PlayPage/hooks/useReleases.ts`
- `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts`
- `cat-launcher/src/pages/SettingsPage/hooks/useColorThemes.ts`
- `cat-launcher/src/pages/SettingsPage/hooks/useFonts.ts`
- `cat-launcher/src/pages/TilesetsPage/hooks.ts`
- `cat-launcher/src/pages/SoundpacksPage/hooks.ts`
- `cat-launcher/src/pages/AchievementsPage/hooks/useAchievements.ts`

**Suggested Fix**: Update these hooks to accept optional error callbacks and use the `useRef` + `useEffect` pattern as shown in the `AGENTS.md` example.

### 2. Internationalization (Strings)
**Inconsistency**: Hardcoded strings are present in UI components.
**Files**:
- `cat-launcher/src/components/VariantSelector.tsx` ("Select a game variant", "Loading...")
- `cat-launcher/src/components/GameSessionMonitor.tsx` ("Game crashed", etc.)
- `cat-launcher/src/components/SearchInput.tsx` ("Search...", "Search")
**Suggested Fix**: Move these strings to an internationalization system or at least ensure they are prepared for it.

### 3. Styling (Manual CSS)
**Inconsistency**: Some components use manual `style` attributes for non-dynamic styling.
**Files**:
- `cat-launcher/src/components/virtualized-combobox.tsx`
- `cat-launcher/src/pages/ModsPage/ModInstallationConfirmationDialog.tsx`
**Suggested Fix**: Prefer Tailwind classes where possible.

## Backend

### 1. Error Handling (Naming Convention)
**Inconsistency**: Error enums in commands are often named `{Feature}CommandError` or `{Feature}Error` instead of the mandatory `{FunctionName}Error`.
**Files**:
- `cat-launcher/src-tauri/src/active_release/commands.rs` (`ActiveReleaseCommandError` -> `GetActiveReleaseError`)
- `cat-launcher/src-tauri/src/settings/commands.rs` (`GetColorThemesCommandError` -> `GetColorThemesError`)
- `cat-launcher/src-tauri/src/mods/commands.rs`
**Suggested Fix**: Rename error enums in `commands.rs` files to exactly match `{FunctionName}Error`.

### 2. Commands (Business Logic Leakage)
**Inconsistency**: Some commands contain business logic that should be in separate files.
**Files**:
- `cat-launcher/src-tauri/src/mods/commands.rs` (contains logic for event emission and path construction)
**Suggested Fix**: Move business logic to dedicated files/functions and keep commands straightforward.
