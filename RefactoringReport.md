# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`. It also includes the suggested fixes for each inconsistency.

## Frontend

### 1. Raw `useQuery` and `useMutation` hooks are used

- **File:** `cat-launcher/src/hooks/useGameVariants.ts`
- **Inconsistency:** The file uses `useQuery` and `useMutation` hooks directly. Instead, custom hooks that wrap `useQuery` and `useMutation` should be created.
- **Fix:** Create custom hooks that wrap `useQuery` and `useMutation` and use them in `useGameVariants.ts`.

### 2. Strings are not internationalization-ready

- **File:** `cat-launcher/src/pages/PlayPage/index.tsx`
- **Inconsistency:** The file contains strings that are displayed to the user but are not internationalization-ready. For example, "Loading..." and "Error: ".
- **Fix:** Use a library like `react-i18next` to make the strings internationalization-ready.

### 3. Overly specific options interface

- **File:** `cat-launcher/src/hooks/useGameVariants.ts`
- **Inconsistency:** The `UseGameVariantsOptions` interface is too specific. It has `onOrderUpdateError` and `onFetchError` properties, which are only used in this hook.
- **Fix:** Generalize the options interface to be more reusable. For example, it could be `useQueryOptions` and `useMutationOptions`.

## Backend

### 1. Commands pass `AppHandle` to business logic

- **File:** `cat-launcher/src-tauri/src/lib.rs`
- **Inconsistency:** The `confirm_quit` command passes the `AppHandle` to the business logic.
- **Fix:** The command should not pass the `AppHandle` to the business logic. Instead, it should call `app_handle.exit(0)` directly.

- **File:** `cat-launcher/src-tauri/src/launch_game/commands.rs`
- **Inconsistency:** The `launch_game` command passes the `AppHandle` to the business logic.
- **Fix:** The command should not pass the `AppHandle` to the business logic. Instead, it should prepare the arguments to pass to the business logic function.

### 2. Error enums use `Box<dyn std::error::Error + Send + Sync>`

- **File:** `cat-launcher/src-tauri/src/play_time/repository.rs`
- **Inconsistency:** The `PlayTimeRepositoryError` enum uses `Box<dyn std::error::Error + Send + Sync>` for some of its variants.
- **Fix:** Define a specific error for each variant.
