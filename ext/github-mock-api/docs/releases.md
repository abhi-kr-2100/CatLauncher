# Mocking the Releases Endpoints

This document outlines the design and usage of the `Release` module in `github-mock-api`. The module allows users to mock the GitHub [Releases API](https://docs.github.com/en/rest/releases/releases?apiVersion=2026-03-10) read-only endpoints: listing releases, getting a release by ID, getting a release by tag name, and getting the latest release.

---

## The `Release` Builder API

Constructing all the fields of a GitHub release manually is tedious. To solve this, `Release::new` automatically populates standard defaults and URLs based on the repository owner, name, and tag name.

### Example Construction

```rust
let release = Release::new("octocat", "hello-world", "v1.0.0")
    .name("v1.0.0")
    .body("Release of my awesome project")
    .target_commitish("main");
```

---

## Usage Example

Below is a concise example of constructing a release and registering it with the mock server.

```rust
// Construct a mocked release
let mocked_release = Release::new("octocat", "hello-world", "v1.0.0")
    .name("v1.0.0")
    .body("Release of my awesome project");

// Register it with the server
server.add_release("octocat", "hello-world", mocked_release).await;

// The release is now available at:
//   GET /repos/octocat/hello-world/releases
//   GET /repos/octocat/hello-world/releases/{release_id}
//   GET /repos/octocat/hello-world/releases/tags/v1.0.0
//   GET /repos/octocat/hello-world/releases/latest
```
