# Refactoring Report

This report details inconsistencies found in the codebase, violating the standards outlined in `AGENTS.md`. Each section describes the issue and suggests a fix.

## Frontend

### 1. Hardcoded Strings

- **File:** `cat-launcher/src/pages/PlayPage/index.tsx`
- **Inconsistency:** The file contains hardcoded strings ("Loading...", "Error: ", "Unknown error", and "Failed to update game variants order") that are not internationalized. `AGENTS.md` requires all user-facing strings to be i18n-ready.
- **Suggested Fix:** Replace the hardcoded strings with keys for an internationalization library (like `react-i18next`).

### 2. Manual CSS

- **File:** `cat-launcher/src/styles/global.css`
- **Inconsistency:** This file contains manual CSS, including theme definitions and base styles. `AGENTS.md` explicitly states to "never write manual CSS" and to prefer Tailwind for styling.
- **Suggested Fix:** Refactor the theme definitions and base styles to use Tailwind's configuration and utility classes. The theming variables should be moved to `tailwind.config.js`, and the base styles should be applied using Tailwind's `@layer` directives or plugins.

## Backend

### 1. Repository Pattern Violation

- **Files:**
  - `cat-launcher/src-tauri/src/fetch_releases/commands.rs`
  - `cat-launcher/src-tauri/src/active_release/commands.rs`
- **Inconsistency:** Commands directly depend on the concrete `SqliteReleasesRepository` and `SqliteActiveReleaseRepository` implementations instead of an abstract repository trait. This violates the repository pattern guidelines in `AGENTS.md`, which require business logic to be decoupled from the database implementation.
- **Suggested Fix:** Introduce a `ReleasesRepository` and `ActiveReleaseRepository` trait, and have the commands depend on these traits instead of the concrete SQLite implementations. The business logic should be updated to use the traits, and the SQLite repositories should implement them.

### 2. Business Logic in Commands

- **File:** `cat-launcher/src-tauri/src/fetch_releases/commands.rs`
- **Inconsistency:** The `fetch_releases_for_variant` command passes the Tauri `AppHandle` to the business logic via a closure. This violates the rule in `AGENTS.md` that framework-dependent objects should not be passed to business logic.
- **Suggested Fix:** Refactor the command to keep the business logic separate from the Tauri-specific code. The command should call the business logic, receive the result, and then emit the event itself, rather than passing the `AppHandle` down.
