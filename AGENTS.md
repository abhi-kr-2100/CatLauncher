# Agent Instructions for CatLauncher

This document provides instructions for AI agents working on the CatLauncher project.

## Project Overview

CatLauncher is an opinionated cross-platform launcher for Cataclysm games with modern social features. It is built with Tauri, using Rust for the backend and a web-based frontend.

## Technology Stack

- **Backend:** Rust with Tauri, `ts-rs` for TypeScript type generation
- **Frontend:** React, TypeScript, and shadcn/ui
- **Package Manager:** pnpm

## Project Structure

- `cat-launcher/`: The main project directory.
  - `src/`: Frontend source code.
  - `src-tauri/`: Backend (Rust) source code.
    - `src/main.rs`: The main entry point for the Rust application.
    - `tauri.conf.json`: Tauri configuration file.
- `README.md`: Project README file.


## ts-rs: TypeScript Type Generation

This project uses the `ts-rs` crate to automatically generate TypeScript type definitions (`.d.ts` files) from Rust structs and enums. This ensures that the frontend and backend types are always in sync.

### How it Works
The type generation is integrated into the testing process. When you run the backend tests, a test case specifically for exporting types is executed. This test will:
1.  Find all Rust types decorated with `#[ts(export)]`.
2.  Generate the corresponding TypeScript types.
3.  Save them to a gitignored directory within the frontend's source tree.

### Agent Responsibility
If you modify any Rust structs or enums that are shared with the frontend (i.e., they have a `#[ts(export)]` attribute), you **must** run the backend tests to regenerate the TypeScript types:
```bash
cargo test --manifest-path ./cat-launcher/src-tauri/Cargo.toml
```
This ensures the frontend has access to the latest type definitions.

## Running Tests

The project has both frontend and backend tests.

- **Backend Tests:**
  ```bash
  cargo test --manifest-path ./cat-launcher/src-tauri/Cargo.toml
  ```

### Checking for Syntax Errors
To quickly check your Rust code for syntax errors without compiling, you can run the following command:
  ```bash
  cargo check --manifest-path ./cat-launcher/src-tauri/Cargo.toml
  ```

- **Frontend Tests:**
  - No frontend tests have been set up yet.

## Agent Workflow

When working on this project, please follow these guidelines:

1.  **Understand the Architecture:** Before making changes, familiarize yourself with the Tauri architecture, which separates the backend (Rust) and frontend (web view).
2.  **Perform Self-Review:** After making any change, run `cr review --plain -t uncommitted` if the command is available. Address any feedback provided before proceeding.
3.  **Run Tests:** After making any changes, run the relevant tests to ensure that you haven't introduced any regressions.
4.  **Update Documentation:** If you add or modify features, update this `AGENTS.md` file and the `README.md` as needed.
5.  **Keep Dependencies Updated:** Regularly check for and update outdated dependencies.
6.  **Follow Existing Conventions:** Adhere to the coding style and conventions already present in the codebase.