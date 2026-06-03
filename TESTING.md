# Testing `fetch_releases`

This document outlines the test scenarios for the `fetch_releases` function in `cat-launcher/src-tauri/src/fetch_releases/fetch_releases.rs`.

## Functionality Overview

The `fetch_releases` function performs the following steps:
1.  **Fetch from Cache**: Retrieves cached releases from the SQLite database and emits them with a `Fetching` status.
2.  **Fetch from GitHub**: Fetches the latest 100 releases from the GitHub API.
3.  **Update Cache**: Saves the fetched releases to the SQLite database.
4.  **Emit GitHub Releases**: Emits the newly fetched releases with a `Fetching` status.
5.  **Emit Default Releases**: Retrieves default releases from a local JSON file and emits them with a `Success` status.

Releases are filtered by platform (OS and Architecture) before being emitted.

## Test Scenarios

### 1. Happy Path: Fresh Fetch
- **Setup**: Empty database, mock GitHub API with several installable releases.
- **Action**: Call `fetch_releases`.
- **Expected Results**:
  - First emission: Empty list of releases (nothing in cache).
  - Second emission: List of releases fetched from GitHub.
  - Third emission: List of default releases.
  - Database: `releases` table should contain the fetched releases.

### 2. Happy Path: Cached Fetch
- **Setup**: Database with existing releases, mock GitHub API with updated releases.
- **Action**: Call `fetch_releases`.
- **Expected Results**:
  - First emission: List of cached releases.
  - Second emission: List of updated releases from GitHub.
  - Third emission: List of default releases.
  - Database: `releases` table should be updated with new releases.

### 3. Platform Filtering
- **Setup**: Mock GitHub API with releases for various platforms (Linux, Windows, Mac).
- **Action**: Call `fetch_releases` for a specific OS and Arch (e.g., Linux x86_64).
- **Expected Results**:
  - The emitted payloads should only contain releases that have assets matching the specified platform.

### 4. GitHub API Error
- **Setup**: Mock GitHub API returns an error (e.g., 404 Not Found).
- **Action**: Call `fetch_releases`.
- **Expected Results**:
  - Function should return a `FetchReleasesError::Fetch`.
  - Cached releases should still be emitted before the error occurs.

### 5. SQLite Database Error
- **Setup**: Mock the database repository to return an error.
- **Action**: Call `fetch_releases`.
- **Expected Results**:
  - Function should return a `FetchReleasesError::Repository`.

## Limitations of `github-mock-api`

- **Pagination**: The `github-mock-api` does not currently support the `Link` header used by GitHub for pagination. While `fetch_releases` only requests up to 100 releases (which fits in a single page), any test requiring multiple pages of GitHub releases cannot be fully implemented.
- **Rate Limiting**: The mock server does not simulate GitHub's rate limiting headers or behavior.
- **Authentication**: The mock server does not verify GitHub personal access tokens.
- **Complex Queries**: Advanced query parameters for filtering releases might not be supported.
