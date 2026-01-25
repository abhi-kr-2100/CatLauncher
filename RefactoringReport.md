# Refactoring Report

This report documents the inconsistencies found in the codebase and the fixes applied to align with the coding standards outlined in `AGENTS.md`.

## Frontend

### Inconsistency: Raw `useQuery` and `useMutation` Hooks

The `AGENTS.md` file specifies that raw `useQuery` and `useMutation` hooks should not be used directly. Instead, they should be wrapped in custom hooks that handle errors with callbacks.

**Files affected:**

- `cat-launcher/src/hooks/useBackups.ts`
- `cat-launcher/src/hooks/useGameVariants.ts`

### Fix: Wrapped Hooks in Custom Hooks

I refactored the affected hooks to wrap the `useQuery` and `useMutation` hooks and accept `onError` callbacks, as per the guidelines. This ensures that errors are handled consistently across the application.

## Backend

### Inconsistency: Business Logic in Command

The `AGENTS.md` file specifies that Tauri commands should be straightforward and should not perform business logic. Instead, they should delegate to separate business logic functions.

**File affected:**

- `cat-launcher/src-tauri/src/active_release/commands.rs`

### Fix: Extracted Business Logic

I moved the business logic from the `get_active_release` command into a new function in `cat-launcher/src-tauri/src/active_release/active_release.rs`. The command now calls this new function, maintaining a clean separation of concerns.
