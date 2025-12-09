# Agent Instructions for CatLauncher

This document provides a high-level overview of the CatLauncher project for AI agents. Its purpose is to guide you in understanding the architecture and finding more detailed documentation.

## Technologies

The project uses a modern technology stack. For a comprehensive overview, please see the [Project Overview](./dev-docs/project-overview.md) document. Key technologies include:

-   **Backend:** Rust, Tauri
-   **Frontend:** TypeScript, React, TanStack Query
-   **Styling:** Tailwind CSS, shadcn/ui
-   **Database:** SQLite

## Core Principles

### Vertical Slice Architecture
The project is organized by features, not layers. Both the frontend and backend follow this principle, meaning that all the code for a single feature is located in the same directory.

-   **Backend Feature Directory:** A typical backend feature slice (e.g., `src-tauri/src/fetch_releases/`) contains:
    -   `mod.rs`: The module declaration file.
    -   `commands.rs`: The bridge to the Tauri framework.
    -   A file with the core business logic (e.g., `fetch_releases.rs`).
    -   `repository.rs`: The trait definition for the repository.
    -   `sqlite_repository.rs`: The SQLite implementation of the repository trait.

-   **Frontend Feature Directory:** A typical frontend feature slice (e.g., `src/pages/PlayPage/`) contains:
    -   `PlayPage.tsx`: The main page component.
    -   `hooks.tsx`: Custom hooks related to the page.
    -   `utils.tsx`: Utility functions specific to the page.
    -   `constants.tsx`: Constants used on the page.

### Clean Architecture
The backend's business logic is strictly decoupled from the framework.

-   **Framework Components:** Anything that is not pure business logic is considered part of the "framework." This includes:
    -   Tauri (the application framework itself)
    -   SQLite repository implementations
    -   The operating system (OS) abstraction
    -   System time
    -   Filesystem paths

-   **Interaction Boundaries:**
    -   Interaction with Tauri happens exclusively in `commands.rs` files.
    -   Business logic functions **never** depend directly on the SQLite implementation of a repository. Instead, they depend on a `repository` trait (an abstract interface).
    -   All framework-level dependencies (OS, time, paths, repository implementations) are passed as parameters to business logic functions from the `commands.rs` functions.

## Coding Conventions

### Backend (Rust)

-   **Error Handling:** `thiserror` is used for creating custom error types. The `anyhow` crate is not used. Each function defines its own error enum. The `#[from]` attribute is used to wrap lower-level errors within higher-level ones. Internal errors must be strongly-typed and not simple strings.

-   **Database:** The repository pattern is used to abstract database access. A `repository.rs` file defines a trait with the methods needed for data access. Business logic functions depend only on this trait, not on any specific implementation.

-   **Type Generation:** `ts-rs` is used to automatically generate TypeScript type definitions from Rust structs and enums.
    -   Types are generated into the `src/generated-types/` directory in the frontend.
    -   To regenerate the types after modifying a Rust struct with `#[ts(export)]`, you must run the backend tests: `cargo test --manifest-path ./cat-launcher/src-tauri/Cargo.toml`.

### Frontend (TypeScript/React)

-   **Data Fetching:** `@tanstack/react-query` is used for all asynchronous operations and state caching.
    -   Query keys are centrally managed in `src/lib/queryKeys.ts`. Using string literals or "magic literals" for query keys is not allowed; always import them from the central file.

-   **Styling:** Tailwind CSS and `shadcn/ui` are used for styling and UI components.
    -   To add new `shadcn/ui` components, you must use the CLI: `pnpx shadcn-ui@latest add <component_name>`.

## Detailed Documentation

For more in-depth information on the architecture, modules, and best practices, please refer to the documents in the `dev-docs` directory.
-   [Project Overview](./dev-docs/project-overview.md)
-   [Backend Architecture](./dev-docs/backend-architecture.md)
-   [Frontend Architecture](./dev-docs/frontend-architecture.md)
-   [Best Practices](./dev-docs/best-practices.md)
