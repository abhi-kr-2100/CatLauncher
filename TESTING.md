# Testing fetch_release_notes

This document outlines the test scenarios for the `fetch_release_notes` function in `cat-launcher/src-tauri/src/fetch_releases/fetch_releases.rs`.

## Scenarios

### 1. Cache Hit with Body
- **Description**: The release is already in the SQLite database and has a non-empty release notes body.
- **Expected Behavior**:
    - The function returns the cached body.
    - No GitHub API call is made.
- **Verification**: Ensure the returned body matches the cached one and no request is sent to the mock GitHub server.

### 2. Cache Hit with Empty Body
- **Description**: The release is in the SQLite database, but its `body` field is `None`.
- **Expected Behavior**:
    - The function fetches the release from GitHub.
    - The cache is updated with the fetched release (including the body).
    - The function returns the fetched body.
- **Verification**: Check that the body returned matches GitHub's response and the database now contains the body.

### 3. Cache Miss
- **Description**: The release is not present in the SQLite database.
- **Expected Behavior**:
    - The function fetches the release from GitHub.
    - The cache is updated with the fetched release.
    - The function returns the fetched body.
- **Verification**: Check that the body returned matches GitHub's response and the release is now in the database.

### 4. GitHub Release Not Found (404)
- **Description**: The release is not in the cache, and GitHub returns a 404 Not Found error.
- **Expected Behavior**:
    - The function returns a `FetchReleaseNotesError::Fetch` containing the HTTP error.
- **Verification**: Assert that an error is returned and it matches the expected error variant.

### 5. GitHub API Error (500)
- **Description**: GitHub returns a 500 Internal Server Error.
- **Expected Behavior**:
    - The function returns a `FetchReleaseNotesError::Fetch`.
- **Verification**: Assert that an error is returned.

### 6. Different Game Variants
- **Description**: Test the function with different `GameVariant` values (e.g., `DarkDaysAhead`, `BrightNights`).
- **Expected Behavior**:
    - The function correctly identifies the GitHub repository for each variant and fetches notes from the correct place.
- **Verification**: Verify that the correct repository URL is hit on the mock server.

### 7. Tag Name with Special Characters
- **Description**: Test with a tag name that contains characters needing URL encoding (e.g., `v1.0.0+test`).
- **Expected Behavior**:
    - The tag name is correctly encoded in the GitHub API request.
- **Verification**: Verify the mock server receives the correctly encoded tag name.

## Request Verification

- **Request Verification**: While `github-mock-api` does not provide built-in request verification, `TestHttpClient` in `cat-launcher/src-tauri/src/infra/testing/http_client.rs` has been enhanced with a request counter.
    - `request_count()`: Returns the number of GET requests made through the client.
    - `reset_request_count()`: Resets the counter to zero.
- **Verification strategy**: Tests use these methods to assert that no network calls are made during cache hits and exactly one call is made when fetching from GitHub is required.

## `github-mock-api` Limitations

- **Error Injection**: `github-mock-api` supports injecting specific HTTP errors (like 500 Internal Server Error) using `MockBehavior`.

## Test Environment Setup

- **Fresh Mock Server**: Each test starts its own `MockServer` on a random port to ensure isolation.
- **Fresh Test Database**: Each test uses `TestDatabaseBuilder` to create a new temporary SQLite database.
- **Host Mapping**: `TestHttpClient` is configured to map `api.github.com` to the local `MockServer` address.
