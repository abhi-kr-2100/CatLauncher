# Testing `fetch_release_notes`

This document outlines the test scenarios for the `fetch_release_notes` function in `cat-launcher/src-tauri/src/fetch_releases/fetch_releases.rs`.

## Test Scenarios

### 1. Cache Hit (Body Present)
- **Description**: The release is already present in the SQLite database and contains a non-empty body.
- **Expected Outcome**: The function returns the cached body immediately without making a call to the GitHub API.
- **Verification**:
    - Assert the returned body matches the database record.
    - Ensure the mock GitHub server receives no requests (if possible to verify) or simply doesn't need to have the data.

### 2. Cache Hit (Body Missing)
- **Description**: The release record exists in the database, but the `body` field is `NULL` or missing.
- **Expected Outcome**: The function fetches the release from GitHub, updates the database with the new body, and returns it.
- **Verification**:
    - Assert the returned body matches what the mock GitHub server provided.
    - Verify the database now contains the body for that release.

### 3. Cache Miss (GitHub Hit)
- **Description**: The release is not found in the database.
- **Expected Outcome**: The function fetches the release from GitHub, creates a new entry in the database (including assets, etc.), and returns the body.
- **Verification**:
    - Assert the returned body matches the mock GitHub server.
    - Verify the database now contains the full release record.

### 4. GitHub 404 (Not Found)
- **Description**: The release is not in the database and not on GitHub.
- **Expected Outcome**: The function returns a `FetchReleaseNotesError::Fetch` error, wrapping an `HttpClientError` with a 404 status.
- **Verification**:
    - Assert the result is an error.
    - Verify the error type and status code.

### 5. GitHub 500 (Internal Server Error)
- **Description**: The release is not in the database, and GitHub returns a 500 error.
- **Expected Outcome**: The function returns an error.
- **Verification**:
    - (See limitations below)

## `github-mock-api` Limitations

The following limitations were identified in the `github-mock-api` external repository:

1. **State-based Mocking**: The mock server is primarily state-based. You add `Release` objects to the state, and the server provides handlers for `list`, `get`, etc. It does not easily allow mocking arbitrary HTTP response codes (like 500, 429, or 403) for specific calls unless the handler itself produces them (e.g., 404 when a release is missing).
    - *Impact*: Scenario 5 (GitHub 500) cannot be easily implemented using only `github-mock-api`.
2. **Hardcoded Base URL**: The production code in `fetch_releases.rs` (via `infra/github/utils.rs`) uses hardcoded `https://api.github.com` URLs.
    - *Workaround*: A custom `HttpClient` implementation for tests must be used to redirect these requests to the local mock server's URI.
3. **Missing Fields (Assets)**: `github-mock-api` uses `#[serde(skip_serializing_if = "Vec::is_empty")]` for the `assets` field. However, the production code's `GitHubRelease` struct (in `src/infra/github/release.rs`) expects the `assets` field to be present in the JSON response. This causes a "missing field `assets`" parse error when the mock server returns a release with no assets.
    - *Impact*: Scenarios 2 and 3 cannot be fully verified because any successful fetch from the mock GitHub API fails during deserialization in the production code.
