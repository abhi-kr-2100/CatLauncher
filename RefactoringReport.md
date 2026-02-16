# Refactoring Report

This report identifies inconsistencies in the codebase according to the coding standards and advice mentioned in `AGENTS.md`.

## Identified Inconsistencies

### Frontend

#### 1. Directory Structure
- **Violation:** `cat-launcher/src/pages/AboutPage.tsx` is a file instead of a directory with `index.tsx`.
  - **Suggested Fix:** Move to `cat-launcher/src/pages/AboutPage/index.tsx`.
- **Violation:** `cat-launcher/src/pages/game-tips/` is lowercase and missing an `index.tsx`.
  - **Suggested Fix:** Rename to `GameTips` (for consistency), move `TipOfTheDay.tsx` to `components/TipOfTheDay.tsx`, and create `index.tsx`.

#### 2. Internationalization (i18n) Readiness
- **Violation:** Manual string construction for dates in `cat-launcher/src/pages/BackupsPage/index.tsx` and `columns.tsx`.
  - **Suggested Fix:** Use `Intl.DateTimeFormat`.
- **Violation:** String manipulation (`toUpperCase`) for display labels in `cat-launcher/src/pages/BackupsPage/columns.tsx`.
  - **Suggested Fix:** Use a mapping function or constant.
- **Violation:** Hardcoded strings in multiple pages.
  - **Suggested Fix:** While no i18n framework is present, strings should be prepared for it by avoiding construction in parts and using constants/mappings where appropriate.

#### 3. Data Fetching & Hooks
- **Violation:** Raw `useQuery` used in `cat-launcher/src/providers/PostHogProviderWithIdentifiedUser.tsx`.
  - **Suggested Fix:** Wrap in a custom hook.
- **Violation:** Many custom hooks do not follow the recommended `useRef` + `useEffect` pattern for error callbacks.
  - **Suggested Fix:** Refactor hooks in `cat-launcher/src/hooks/` and `cat-launcher/src/pages/*/hooks/` to use the recommended pattern.

### Backend

#### 1. Command Organization
- **Violation:** `confirm_quit` command is defined in `cat-launcher/src-tauri/src/lib.rs` instead of a feature-specific `commands.rs`.
  - **Suggested Fix:** Move to a relevant feature module (e.g., `app`).

#### 2. Error Handling
- **Violation:** `confirm_quit` command does not return a `Result` or use the standard error pattern.
  - **Suggested Fix:** Refactor to return `Result<(), Error>` and use `thiserror`.

## Applied Fixes

- [x] Fix Frontend Directory Structure
- [x] Improve Frontend i18n Readiness
- [x] Standardize Frontend Hooks
- [x] Fix Backend Command and Error Handling
