# Agent Instructions for CatLauncher

This document provides instructions for AI agents working on the CatLauncher project.

## Project Overview

CatLauncher is an opinionated cross-platform launcher for Cataclysm games with modern social features. It is built with Tauri, using Rust for the backend and a web-based frontend.

## Architecture

The project's architecture is designed for maintainability and testability, with a clear separation between the backend (Rust) and frontend (React).

### Backend Architecture

The backend follows a **Vertical Slice Architecture**. Each feature (e.g., `fetch_releases`, `install_release`) is encapsulated within its own module in `src-tauri/src/`. This modular approach keeps all related logic for a feature self-contained.

Within each slice, the principles of **Clean Architecture** are applied:
- **Framework Agnostic Core:** The core business logic (e.g., in `fetch_releases.rs`) is decoupled from the Tauri framework. It receives dependencies like paths and HTTP clients via arguments (dependency injection), making it easy to test in isolation.
- **Framework Bridge:** The `commands.rs` file in each module is the only part that interacts directly with Tauri. It acts as a bridge, exposing the core logic to the frontend as Tauri commands.

### Frontend-Backend Interaction

The frontend communicates with the backend by invoking the commands defined in the Rust `commands.rs` files. To keep the UI components decoupled from the backend implementation:
- All Tauri `invoke` calls are wrapped in async functions inside `src/lib/utils.ts`.
- UI components call these wrapper functions, remaining unaware of the underlying Tauri API. This makes the components more reusable and easier to test.

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

1.  **Understand the Architecture:** Before making changes, familiarize yourself with the project's architecture as described in this document.
2.  **Maintain Architecture:** When adding new features or modifying existing code, adhere to the established architectural patterns. For example, new backend features should be implemented as new vertical slices, and frontend-backend communication should continue to use the wrappers in `lib/utils.ts`.
3.  **Perform Self-Review:** After making any change, run `cr review --plain -t uncommitted` if the command is available. Address any feedback provided before proceeding.
4.  **Run Tests:** After making any changes, run the relevant tests to ensure that you haven't introduced any regressions.
5.  **Update Documentation:** If you add or modify features, update this `AGENTS.md` file and the `README.md` as needed.
6.  **Keep Dependencies Updated:** Regularly check for and update outdated dependencies.
7.  **Follow Existing Conventions:** Adhere to the coding style and conventions already present in the codebase.
