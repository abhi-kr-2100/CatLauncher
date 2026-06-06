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

## Vendored Repositories

Vendored repositories are dependencies included with this project under the ext/ directory.

- The ext/ directory is a read-only reference.
- Do not import from ext/.
