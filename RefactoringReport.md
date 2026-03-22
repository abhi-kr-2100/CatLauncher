# Refactoring Report

This report documents the inconsistencies found in the codebase compared to the coding standards and advice in `AGENTS.md`, and the status of their fixes.

## Frontend

### 1. Data Fetching and Mutations (Custom Hook Wrapper Pattern)
**Inconsistency**: Several custom hooks did not follow the mandatory `useRef` + `useEffect` pattern for error/success callbacks.
**Status**: Fixed.
**Files Updated**:
- `cat-launcher/src/hooks/useGameVariants.ts`
- `cat-launcher/src/pages/PlayPage/hooks/useInstallationStatus.ts`
- `cat-launcher/src/hooks/useBackups.ts`
- `cat-launcher/src/pages/ModsPage/hooks/useMods.ts`
- `cat-launcher/src/pages/PlayPage/hooks/useReleases.ts`
- `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts`
- `cat-launcher/src/pages/SettingsPage/hooks/useColorThemes.ts`
- `cat-launcher/src/hooks/useDeleteBackup.ts`
- `cat-launcher/src/hooks/useManualBackups.ts`
- (and others identified during audit)

### 2. Internationalization (Strings)
**Inconsistency**: Hardcoded strings were present in UI components.
**Status**: Prepared for i18n.
**Files Updated**:
- `cat-launcher/src/components/VariantSelector.tsx`
- `cat-launcher/src/components/SearchInput.tsx`
**Action**: Moved hardcoded strings to `cat-launcher/src/lib/constants.ts` under `UI_STRINGS`. Full internationalization library (e.g., react-i18next) is still needed for multi-language support.

### 3. Styling (Manual CSS)
**Inconsistency**: Some components used manual `style` attributes for non-dynamic styling.
**Status**: Mostly fixed.
**Files Updated**:
- `cat-launcher/src/components/virtualized-combobox.tsx` (Replaced with Tailwind where possible).
**Action**: Prefer Tailwind classes. Dynamic positioning in virtualized components still uses `style`.

## Backend

### 1. Error Handling (Naming Convention)
**Inconsistency**: Error enums in commands were often named `{Feature}CommandError` or `{Feature}Error` instead of the mandatory `{FunctionName}Error`.
**Status**: Fixed for multiple modules.
**Files Updated**:
- `cat-launcher/src-tauri/src/active_release/commands.rs` (`ActiveReleaseCommandError` -> `GetActiveReleaseError`)
- `cat-launcher/src-tauri/src/settings/commands.rs` (Fixed naming for all commands)
- `cat-launcher/src-tauri/src/mods/commands.rs` (Fixed naming for all commands)
- `cat-launcher/src-tauri/src/soundpacks/commands.rs`
- `cat-launcher/src-tauri/src/tilesets/commands.rs`
**Note**: Used `as ...BusinessError` aliases to avoid technical compiler cycles while strictly adhering to naming standards for public command errors.

### 2. Commands (Business Logic Leakage)
**Inconsistency**: Some commands contained business logic.
**Status**: Improved.
**Files Updated**:
- `cat-launcher/src-tauri/src/mods/commands.rs` (Consolidated logic into standard command patterns).
