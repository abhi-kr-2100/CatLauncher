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

## `github-mock-api` Limitations

- **No built-in request verification**: The mock server does not provide a way to inspect received requests or count hits.
    - *Workaround*: To verify that NO call was made, we can use a `TestHttpClient` without a mapping for the GitHub API host, which will result in a "No mapping found" error if a call is attempted.
    - *Workaround*: To verify THAT a call was made, we can ensure the data is NOT in the cache and see it successfully return data that could only have come from the mock server.
- **Limited Error Injection**: While we can mock 404s by not adding a release, injecting other specific HTTP errors (like 500) might be limited if the mock server doesn't support it directly.
    - *Note*: If `github-mock-api` cannot return a 500, this scenario will be marked as failed/untestable as per instructions.

## Test Environment Setup

- **Fresh Mock Server**: Each test starts its own `MockServer` on a random port to ensure isolation.
- **Fresh Test Database**: Each test uses `TestDatabaseBuilder` to create a new temporary SQLite database.
- **Host Mapping**: `TestHttpClient` is configured to map `api.github.com` to the local `MockServer` address.
