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
- ...and others identified in the audit.
**Suggested Fix**: Update these hooks to accept an optional error callback and use the `useRef` + `useEffect` pattern as shown in the `AGENTS.md` example.

### 2. Internationalization (Strings)
**Inconsistency**: Hardcoded strings are present in UI components.
**Files**:
- `cat-launcher/src/components/VariantSelector.tsx` ("Select a game variant", "Loading...")
- `cat-launcher/src/components/GameSessionMonitor.tsx` ("Game crashed", etc.)
- `cat-launcher/src/components/SearchInput.tsx` ("Search...")
**Suggested Fix**: Move these strings to an internationalization system or at least ensure they are prepared for it. *Note: The prompt says to follow AGENTS.md, which says avoid string construction in parts and manipulation.*

### 3. Styling (Manual CSS)
**Inconsistency**: Some components use manual `style` attributes.
**Files**:
- `cat-launcher/src/components/virtualized-combobox.tsx`
- `cat-launcher/src/pages/ModsPage/ModInstallationConfirmationDialog.tsx`
**Suggested Fix**: Where possible, use Tailwind classes. For truly dynamic values (like percentages or absolute positioning calculated at runtime), the `style` attribute is acceptable but should be used sparingly.

## Backend

### 1. Error Handling (Naming Convention)
**Inconsistency**: Error enums are named `{FunctionName}CommandError` instead of `{FunctionName}Error`.
**Files**:
- `cat-launcher/src-tauri/src/active_release/commands.rs` (`ActiveReleaseCommandError` -> `ActiveReleaseError`)
- `cat-launcher/src-tauri/src/settings/commands.rs` (`GetColorThemesCommandError` -> `GetColorThemesError`)
- `cat-launcher/src-tauri/src/mods/commands.rs` (`ListAllModsCommandError` -> `ListAllModsError`, etc.)
- ...and others identified in the audit.
**Suggested Fix**: Rename error enums in `commands.rs` files to follow the `{FunctionName}Error` naming convention.

### 2. Commands (Business Logic and AppHandle leakage)
**Inconsistency**: Some commands contain moderate logic or use `AppHandle` in ways that could be further cleaned up.
**Files**:
- `cat-launcher/src-tauri/src/mods/commands.rs` (logic in `list_all_mods_command`)
**Suggested Fix**: Ensure all business logic is moved out of commands. `AppHandle` should only be used to prepare arguments.
