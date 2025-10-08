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
2.  **Run Tests:** After making any changes, run the relevant tests to ensure that you haven't introduced any regressions.
3.  **Update Documentation:** If you add or modify features, update this `AGENTS.md` file and the `README.md` as needed.
4.  **Keep Dependencies Updated:** Regularly check for and update outdated dependencies.
5.  **Handle TypeScript Type Generation:** This project uses the `ts-rs` crate to generate TypeScript types from Rust structs and enums. These generated types are stored in a gitignored directory. If you modify any Rust types that are exported to the frontend, you **must** run the tests (`cargo test --manifest-path ./cat-launcher/src-tauri/Cargo.toml`). The tests are configured to automatically regenerate the TypeScript bindings, ensuring that the frontend has access to the updated types.
6.  **Follow Existing Conventions:** Adhere to the coding style and conventions already present in the codebase.