# AGENTS.md

This file provides guidance to agents when working with code in this repository.

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
