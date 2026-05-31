# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Commit Guidelines

* Do not commit unless asked.
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

BREAKING CHANGE: Authentication is now required for all API endpoints.
```

## Verification

At the end of every task, run the following commands:

* `rm -rf cat-launcher/src/generated-types && cargo test --manifest-path cat-launcher/src-tauri/Cargo.toml`. This will generate TypeScript types from Rust types using `ts-rs`.
* `pnpm --prefix cat-launcher format && pnpm --prefix cat-launcher lint:fix`
* `pnpm --prefix cat-launcher lint` to ensure there are no errors.

## Vendored Repositories

Vendored repositories are dependencies included with this project under the ext/ directory.

* The ext/ directory is a read-only reference.
* Do not import from ext/.

## Project Overview

* This project is called CatLauncher. It is a launcher for Cataclysm Dark Days Ahead and its two most popular forks: Bright Nights and The Last Generation.
* The project is being developed inside a Nix development shell. If a program is missing, ensure the Nix shell is active and retry. See `flake.nix` for all available packages.
* This project uses the Tauri framework with a React frontend. The JavaScript project is inside the `cat-launcher/` directory. The Tauri project is inside the `cat-launcher/src-tauri/` directory, with macros in the `cat-launcher/cat-macros/` directory. Misc scripts are inside the `scripts/` directory.
