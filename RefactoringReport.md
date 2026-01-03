# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`.

## Frontend

### Directory Structure

The `AGENTS.md` file specifies the following directory structure for features inside `cat-launcher/src/pages`:

```
cat-launcher/src/pages/{feature_name}/
  - index.tsx
  - components/
  - hooks/
  - lib/
  - store/
```

The following pages do not follow this structure:

1.  **`AboutPage.tsx`**
    *   **Inconsistency:** It is a file instead of a directory.
    *   **Suggested Fix:**
        *   Create a new directory `cat-launcher/src/pages/AboutPage`.
        *   Move the contents of `cat-launcher/src/pages/AboutPage.tsx` to `cat-launcher/src/pages/AboutPage/index.tsx`.

2.  **`AssetsPage`**
    *   **Inconsistency:** `AssetTypeSelector.tsx` is in the root of the feature directory. `types.ts` is also in the root.
    *   **Suggested Fix:**
        *   Create a `components` directory inside `AssetsPage`.
        *   Move `AssetTypeSelector.tsx` to the new `components` directory.
        *   Create a `lib` directory and move `types.ts` into it.

3.  **`BackupsPage`**
    *   **Inconsistency:** `BackupFilter.tsx`, `BackupsTable.tsx`, `DeleteBackupDialog.tsx`, `NewBackupDialog.tsx`, `RestoreBackupDialog.tsx`, `columns.tsx` are in the root of the feature directory. It also has a `types` directory.
    *   **Suggested Fix:**
        *   Create a `components` directory inside `BackupsPage`.
        *   Move the component files into the new `components` directory.
        *   Create a `lib` directory and move the `types` directory into it.

4.  **`ModsPage`**
    *   **Inconsistency:** Component files and `hooks.ts` are in the root of the feature directory.
    *   **Suggested Fix:**
        *   Create a `components` directory and move `ModCard.tsx`, `ModInstallationConfirmationDialog.tsx`, and `ModsList.tsx` into it.
        *   Create a `hooks` directory and move `hooks.ts` into it.

5.  **`PlayPage`**
    *   **Inconsistency:** Component files and `hooks.ts` are in the root of the feature directory.
    *   **Suggested Fix:**
        *   Create a `components` directory and move `GameVariantCard.tsx`, `InteractionButton.tsx`, `PlayTime.tsx`, `ReleaseFilter.tsx`, `ReleaseLabel.tsx`, and `ReleaseSelector.tsx` into it.
        *   Create a `hooks` directory and move `hooks.ts` into it.


## Backend

### Directory Structure

The `AGENTS.md` file specifies that the backend code should be organized into features, with each feature having a specific internal structure (`mod.rs`, `commands.rs`, `lib.rs`, etc.). The current structure in `cat-launcher/src-tauri/src` is a flat list of directories, and the internal structure of these directories is not consistent with the guidelines.

A full analysis of the backend is pending, but a quick inspection reveals that the prescribed structure is not being followed. Each feature directory should be reviewed and refactored to match the `AGENTS.md` guidelines.
