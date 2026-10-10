# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Project Overview

- CatLauncher is a launcher for Cataclysm Dark Days Ahead and its two most popular forks: Bright Nights and The Last Generation.
- The project is being developed inside a Nix development shell. If a program is missing, ensure the Nix shell is active and retry. See `flake.nix` for all available packages.
- This project uses the Tauri framework with a React frontend.
  - The JavaScript project is inside the `cat-launcher/` directory.
  - The Tauri project is inside the `cat-launcher/src-tauri/` directory.
  - Rust macros are in the `cat-launcher/cat-macros/` directory.
  - Misc scripts are inside the `scripts/` directory.

## Verification

At the end of every task, you must run `task verify` and ensure there are no errors.

## Testing Support

Backend tests have access to testing utilities under `cat-launcher/src-tauri/src/infra/testing/`:

- **Test Database ([`TestDatabase`](cat-launcher/src-tauri/src/infra/testing/test_database.rs))**: Creates ephemeral SQLite database pools initialized with schema migrations and optional preloaded fixtures.
- **Mock GitHub Server (`github_mock_api::MockServer`)**: In-process mock server for GitHub releases, assets, release notes, and error conditions without hitting the network.
- **Test HTTP Client ([`TestHttpClient`](cat-launcher/src-tauri/src/infra/testing/http_client.rs))**: Intercepts HTTP requests and rewrites hostnames (e.g., mapping `api.github.com` to a local `MockServer`).
- **Synthetic Archive Builder ([`TestZip`](cat-launcher/src-tauri/src/infra/testing/test_zip.rs))**: Generates in-memory and on-disk ZIP archives for archive extraction and installation tests.
- **Test Data Setup ([`scripts/setup-test-data`](scripts/setup-test-data))**: Script run automatically via `task test` to fetch and cache test asset fixtures.

## Vendored Repositories

Vendored repositories are dependencies included with this project under the ext/ directory.

- The ext/ directory is a read-only reference.
- Do not import from ext/.

