# Agent Instructions for CatLauncher

This document provides a high-level overview of the CatLauncher project for AI agents. Its purpose is to guide you in understanding the architecture and finding more detailed documentation.

## Core Principles

1.  **Vertical Slice Architecture:** The project is organized by features, not layers. Both the frontend and backend follow this principle. This means that all the code for a single feature is located in the same directory.
2.  **Clean Architecture:** The backend's business logic is decoupled from the framework (Tauri). This is achieved through dependency injection and by separating framework-specific code into dedicated files.
3.  **Frontend/Backend Decoupling:** The frontend and backend are strictly separated. They communicate through commands defined in the Rust backend and wrapped in the frontend.

## Detailed Documentation

For detailed information on the architecture, modules, and best practices, please refer to the documents in the `dev-docs` directory. These documents provide in-depth explanations of the project's design and implementation. Before making any changes, it is highly recommended to review the relevant documents in the `dev-docs` directory.

-   [Project Overview](./dev-docs/project-overview.md)
-   [Backend Architecture](./dev-docs/backend-architecture.md)
-   [Frontend Architecture](./dev-docs/frontend-architecture.md)
-   [Best Practices](./dev-docs/best-practices.md)

## Coding Conventions

### Backend (Rust)

-   **Formatting:** Standard `rustfmt` is used.
-   **Error Handling:** `thiserror` is used for creating custom error types. The use of `anyhow` is discouraged.
-   **Asynchronous Programming:** `async/await` is used extensively for I/O-bound operations.
-   **Database:** The repository pattern is used to abstract database access.
-   **Serialization:** `serde` is used for JSON serialization and deserialization.
-   **Type Generation:** `ts-rs` is used to generate TypeScript types from Rust structs and enums.

### Frontend (TypeScript/React)

-   **Formatting:** Standard Prettier formatting is used.
-   **Data Fetching:** `@tanstack/react-query` is used for managing asynchronous operations, caching, and state.
-   **State Management:** Redux Toolkit is used for global application state.
-   **Styling:** Tailwind CSS and `shadcn/ui` are used for styling and UI components.
-   **Hooks:** Custom hooks are used to encapsulate and reuse component logic.
