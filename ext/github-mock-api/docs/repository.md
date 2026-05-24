# Mocking the "Get a Repository" Endpoint

This document outlines the design and usage of the `Repository` module in `github-mock-api`. The module allows users to mock the GitHub [Get a Repository](https://docs.github.com/en/rest/repos/repos?apiVersion=2026-03-10#get-a-repository) API endpoint.

---

## The `Repository` Builder API

Constructing all 70+ fields of a GitHub repository manually is tedious. To solve this, `Repository::new` automatically populates standard defaults and URLs based on the owner and name.

### Example Construction

```rust
let repo = Repository::new("octocat", "hello-world")
    .description("My awesome repository")
    .private(true)
    .stargazers_count(42)
    .default_branch("develop");
```

---

## Usage Example

Below is the concise example of constructing a repository and registering it with the mock server.

```rust
// Construct a mocked repository
let mocked_repo = Repository::new("octocat", "hello-world")
    .description("This is a mocked repository!")
    .private(false)
    .stargazers_count(1337)
    .default_branch("main");

// Register it with the server
server.add_repository(mocked_repo).await;
```
