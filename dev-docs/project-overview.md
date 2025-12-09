# Project Overview

## Technology Stack

- **Backend:** Rust with Tauri, `ts-rs` for TypeScript type generation
- **Frontend:** React, Redux Toolkit, TanStack Query, TypeScript, and shadcn/ui
- **Package Manager:** cargo and pnpm

## Project Structure

- `cat-launcher/`: The main project directory.
  - `src/`: Frontend source code.
  - `src-tauri/`: Backend (Rust) source code.
    - `tauri.conf.json`: Tauri configuration file.
- `README.md`: Project README file.

### Key Files and Directories

-   `cat-launcher/`: The root directory for the Tauri project.
    -   `src/`: Contains all the frontend React source code.
        -   `lib/commands.ts`: Wraps all Tauri backend commands, decoupling the UI components from the backend API.
        -   `lib/queryKeys.ts`: Contains constants for TanStack Query keys.
        -   `lib/utils.ts`: Contains utility functions.
        -   `PlayPage/`: An example of a screen-based vertical slice on the frontend.
    -   `src-tauri/`: Contains all the backend Rust source code.
        -   `src/`: The Rust crate for the backend.
            -   `<feature>/`: Each directory here represents a vertical slice of the application's features (e.g., `fetch_releases/`, `install_release/`).
                -   `commands.rs`: The bridge between the backend's core logic and the Tauri framework. This file exposes functions as commands to the frontend.
            -   `main.rs`: The main entry point for the Rust application.
    -   `tauri.conf.json`: The main configuration file for the Tauri application, defining permissions, the development server, and other settings.
    -   `Cargo.toml`: The Rust package manager's manifest file, defining backend dependencies.
    -   `package.json`: The Node.js package manager's manifest file, defining frontend dependencies and scripts.

## Agent Workflow

When working on this project, please follow these guidelines:

1.  **Understand the Architecture:** Before making changes, familiarize yourself with the project's architecture as described in this document.
2.  **Maintain Architecture:** When adding new features or modifying existing code, adhere to the established architectural patterns. For example, new backend features should be implemented as new vertical slices, and frontend-backend communication should continue to use the wrappers in `lib/commands.ts`.
3.  **Perform Self-Review:** After making any change, run `cr review --plain -t uncommitted` if the command is available. Address any feedback provided before proceeding.
4.  **Run Tests:** After making any changes, run the relevant tests to ensure that you haven't introduced any regressions.
5.  **Update Documentation:** If you add or modify features, update this `AGENTS.md` file and the `README.md` as needed.
6.  **Follow Existing Conventions:** Adhere to the coding style and conventions already present in the codebase.
