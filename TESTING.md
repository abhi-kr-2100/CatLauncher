# Testing fetch_releases

This document outlines the test scenarios for the `fetch_releases` function in `cat-launcher/src-tauri/src/fetch_releases/fetch_releases.rs`.

## Test Scenarios

### 1. Success Path - Full Flow
- **Goal**: Verify that `fetch_releases` correctly emits releases at all three stages: cached, GitHub, and default.
- **Scenario**:
    - Cache has 1 release.
    - GitHub has 2 different releases.
    - Default has 1 different release.
- **Expected Outcome**: `on_releases` is called 3 times with the respective sets of releases. The final status should be `Success`.

### 2. Initial Cache Emission
- **Goal**: Verify that the function immediately emits cached releases.
- **Scenario**: Cache has releases, GitHub is slow or empty.
- **Expected Outcome**: First call to `on_releases` contains cached releases.

### 3. GitHub Fetch and Emission
- **Goal**: Verify that GitHub releases are fetched and emitted.
- **Scenario**: GitHub returns a list of releases.
- **Expected Outcome**: Second call to `on_releases` contains GitHub releases with `Fetching` status.

### 4. Database Update after GitHub Fetch
- **Goal**: Verify that fetched releases from GitHub are stored in the repository.
- **Scenario**: GitHub returns releases.
- **Expected Outcome**: The `ReleasesRepository::update_cached_releases` is called with the fetched releases.

### 5. Default Releases Emission
- **Goal**: Verify that default releases are emitted at the end.
- **Scenario**: Resources directory contains a `{variant}.json` file with releases.
- **Expected Outcome**: Third call to `on_releases` contains default releases with `Success` status.

### 6. Platform Filtering (OS/Arch)
- **Goal**: Verify that only installable releases are emitted.
- **Scenario**:
    - GitHub has 2 releases: one with Windows assets, one with Linux assets.
    - Test is run with `OS::Windows`.
- **Expected Outcome**: Only the release with Windows assets is included in the `on_releases` payload.

### 7. Pagination Handling
- **Goal**: Verify that the function can handle multiple pages of GitHub releases (up to the 100 limit).
- **Scenario**: GitHub API returns 30 releases per page and has 4 pages.
- **Expected Outcome**: `fetch_github_releases` fetches 100 releases across 4 pages.

### 8. GitHub API Error Handling
- **Goal**: Verify how the function behaves when the GitHub API returns an error.
- **Scenario**: GitHub API returns a 500 Internal Server Error (using `github-mock-api`'s `MockBehavior`).
- **Expected Outcome**: Function returns `FetchReleasesError::Fetch`.

### 9. Repository Error Handling
- **Goal**: Verify how the function behaves when the database is inaccessible.
- **Scenario**: `get_cached_releases` returns an error (using `TestDatabaseBuilder` with a missing schema).
- **Expected Outcome**: Function returns `FetchReleasesError::Repository`.

### 10. Callback Error Handling
- **Goal**: Verify how the function behaves when the `on_releases` callback fails.
- **Scenario**: `on_releases` returns an error.
- **Expected Outcome**: Function returns `FetchReleasesError::Send`.

### 11. Variant Specificity
- **Goal**: Verify that it fetches releases for the correct variant.
- **Scenario**: Call `fetch_releases` for `BrightNights`.
- **Expected Outcome**: Correct GitHub repo is used, and correct default releases file is read.
