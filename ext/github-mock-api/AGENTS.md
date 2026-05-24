# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Project Overview

* This project implements a mock GitHub API server and library meant for integration testing.
* The project is being developed inside a Nix shell. See `flake.nix` for available programs.
* Run `cargo cov-term`, `cargo cov-lcov`, or `cargo cov-html` to run tests with coverage. Aim for 100% coverage.

## Coding Guidelines

* Avoid using `unwrap()`, `expect()`, and other methods that can cause panics. Use proper error handling with `Result`, `Option`, and the `?` operator instead.

## Commit Guidelines

* Do not commit unless asked to.
* Use `jj diff --git --no-pager` to see uncommitted changes.
* Use `jj desc -m {commit_message}` to commit changes.
* Follow the Conventional Commits format:
  - **Header**: `type(scope): description`
    - **Type**: One of `feat`, `fix`, `docs`, `refactor`, `perf`, `style`, `test`, `chore`, `ci`, `revert`, `build`.
    - **Scope** (optional): The name of the feature or module being modified.
    - **Description**: A brief summary of the change.
  - **Body** (optional): A detailed description of the change. Start with the motivation for the change and then list the changes made.
  - **Footer** (optional): Any additional information about the change, like `BREAKING CHANGE` notices or issue references (e.g., `Closes #123`).

Example:

```
feat(user): add user authentication

Motivation:
- To secure user accounts and provide personalized experiences.

Changes:
- Add a new user model.
- Add a new user repository.
- Add a new user service.
- Add a new user controller.

BREAKING CHANGE: Authentication is now required for all API endpoints.
```
