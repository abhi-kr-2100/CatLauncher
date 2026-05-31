# Mocking the "List Commits" and "Get a Commit" Endpoints

This document outlines the design and usage of the `Commit` module in `github-mock-api`. The module allows users to mock the GitHub [List commits](https://docs.github.com/en/rest/commits/commits?apiVersion=2026-03-10#list-commits) and [Get a commit](https://docs.github.com/en/rest/commits/commits?apiVersion=2026-03-10#get-a-commit) API endpoints.

---

## The `Commit` Builder API

Constructing all the fields of a GitHub commit manually is tedious. To solve this, `Commit::new` automatically populates standard defaults and URLs based on the owner and repo. If no SHA is specified, a unique random SHA is generated automatically.

### Example Construction

```rust
let commit = Commit::new("octocat", "hello-world")
    .message("Fix critical bug")
    .author_name("Mona Octocat")
    .author_email("mona@github.com")
    .additions(10)
    .deletions(3);

// Optionally override the auto-generated SHA
let commit = Commit::new("octocat", "hello-world")
    .sha("abc123def")
    .message("Fix critical bug");
```

---

## Usage Example

Below is the concise example of constructing commits and registering them with the mock server.

### Listing Commits

```rust
// Construct mocked commits (SHA auto-generated)
let commit1 = Commit::new("octocat", "hello-world")
    .message("Initial commit")
    .author_name("Mona Octocat");

let commit2 = Commit::new("octocat", "hello-world")
    .message("Add feature")
    .author_name("Mona Octocat")
    .additions(42);

// Register them with the server
server.add_commit(commit1).await;
server.add_commit(commit2).await;

// GET /repos/octocat/hello-world/commits returns both commits
// GET /repos/octocat/hello-world/commits/{sha} returns a specific commit
```
