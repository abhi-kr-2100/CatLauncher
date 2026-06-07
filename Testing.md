# Testing `install_release`

This document details the test suite for the `install_release` function in `cat-launcher/src-tauri/src/install_release/install_release.rs`.

## Overview

The `install_release` function is responsible for:
1.  Determining the current installation status of a release.
2.  Downloading the appropriate asset if necessary.
3.  Extracting the asset to the correct installation directory.
4.  Setting the release as the active one for its variant.
5.  Cleaning up old installations of the same variant.

## Test Environment

-   **Database:** `TestDatabase` is used to provide an isolated SQLite instance.
-   **Mock Server:** `github-mock-api` is used to mock the GitHub API and asset downloads.
-   **Downloader:** A real `Downloader` instance is used, but configured to point to the `MockServer`.
-   **Filesystem:** `tempfile::TempDir` is used for all data and resource directories to ensure isolation.

## Test Scenarios

### 1. Fresh Installation (`test_install_release_fresh_success`)
-   **Scenario:** A release that is not yet downloaded or installed.
-   **Verification:**
    -   Asset is downloaded from the mock server.
    -   Asset is extracted to the correct versioned directory.
    -   `GameRelease` status updates to `ReadyToPlay`.
    -   Release is set as active in the database.
    -   Progress is reported via the `Reporter`.

### 2. Already Installed (`test_install_release_already_installed`)
-   **Scenario:** A release whose installation directory already exists and contains the game executable.
-   **Verification:**
    -   Function skips download and extraction.
    -   `GameRelease` status updates to `ReadyToPlay`.
    -   Release is set as active in the database.

### 3. Corrupted Redownload (`test_install_release_corrupted_redownloads`)
-   **Scenario:** A release marked with `Corrupted` status.
-   **Verification:**
    -   Function forces a redownload and re-extraction.
    -   `GameRelease` status updates to `ReadyToPlay`.

### 4. Cleanup of Previous Installations (`test_install_release_cleanup_others`)
-   **Scenario:** Installing a new release when an older one already exists.
-   **Verification:**
    -   The old version's installation directory is deleted after the new one is successfully installed.

### 5. Error: No Compatible Asset (`test_install_release_no_compatible_asset`)
-   **Scenario:** A release that has no assets matching the current OS and Architecture.
-   **Verification:**
    -   Returns `ReleaseInstallationError::NoCompatibleAsset`.

### 6. Error: Download Failure (`test_install_release_download_failure`)
-   **Scenario:** Mock server returns a 500 Internal Server Error during download.
-   **Verification:**
    -   Returns `ReleaseInstallationError::Download(_)`.

### 7. Error: Extraction Failure (`test_install_release_extraction_failure`)
-   **Scenario:** The downloaded file is not a valid archive.
-   **Verification:**
    -   Returns `ReleaseInstallationError::Extract(_)`.

## Mock Server Limitations

While `github-mock-api` was successfully used to test the full installation flow, the following limitations were noted:
-   **URL Rewriting:** The `Downloader` uses a standard `reqwest::Client`. To route requests to the mock server, the `browser_download_url` in the seeded `ReleasesRepository` must be manually constructed to point to the mock server's URI.
-   **Dynamic Asset Serving:** Assets must be registered with `add_asset` using the exact same `owner`, `repo`, `tag`, and `filename` as expected in the download path.

## How to Run Tests

From the `cat-launcher/src-tauri` directory:

```bash
cargo test --lib install_release::install_release::tests
```
