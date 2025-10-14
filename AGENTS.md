# Agent Instructions for CatLauncher

This document provides instructions for AI agents working on the CatLauncher project.

## Architecture

The project's architecture is designed for testability, with a clear separation between the backend (Rust) and frontend (React).

### Backend Architecture

The backend follows a **Vertical Slice Architecture**. Each feature (e.g., `fetch_releases`, `install_release`) is encapsulated within its own module in `src-tauri/src/`. This modular approach keeps all related logic for a feature self-contained.

Within each slice, the principles of **Clean Architecture** are applied:
- **Framework Agnostic Core:** The core business logic (e.g., in `fetch_releases.rs`) is decoupled from the Tauri framework. It receives dependencies like paths and HTTP clients via arguments (dependency injection), making it easy to test in isolation.
- **Framework Bridge:** The `commands.rs` file in each module is the only part that interacts directly with Tauri. It acts as a bridge, exposing the core logic to the frontend as Tauri commands.

#### Directory and Path Handling

To maintain a clean separation of concerns and enhance portability, business logic functions should not be tightly coupled to the application's directory structure.

-   **Parameter Passing:** Business logic functions should only receive the top-level directory paths they need as parameters. These are typically:
    -   `cache_dir`: The path to the application's cache directory.
    -   `data_dir`: The path to the application's data directory.
    -   `resources_dir`: The path to the application's resources directory.
-   **Path Construction:** Inside the business logic functions, use the helper functions provided in `src-tauri/src/filesystem/paths.rs` to construct the exact paths to specific files or subdirectories.

This approach ensures that the core logic is not cluttered with path manipulation and can be easily tested by passing in mock directory paths.

### Frontend Architecture

The frontend is a standard React application and follows a similar vertical slice structure as the backend. It is organized by screens (e.g., `PlayPage`).

### Frontend-Backend Interaction

The frontend communicates with the backend by invoking the commands defined in the Rust `commands.rs` files. To keep the UI components decoupled from the backend implementation:
- All Tauri `invoke` calls are wrapped in async functions inside `src/lib/commands.ts`.
- UI components call these wrapper functions, remaining unaware of the underlying Tauri API. This makes the components more reusable and easier to test.
- TanStack Query keys are collected into constants in `src/lib/queryKeys.ts`.

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

## Key Files and Directories

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

## Error Handling

The project has a standardized approach to error handling to ensure that errors are managed gracefully and provide clear feedback to the user and developers.

### Backend Error Handling

The backend employs a two-tier error handling strategy to separate internal business logic errors from the errors exposed to the frontend.

1.  **Internal Business Logic Errors**:
    -   These errors are defined within the core logic files (e.g., `fetch_releases.rs`).
    -   They use `thiserror::Error` for clean error propagation within the backend.
    -   They are **not** intended to be sent to the frontend and therefore do **not** implement `serde::Serialize` or derive `strum::IntoStaticStr`.
    -   Each function has its own error enums.

    *Example from `fetch_releases/fetch_releases.rs`*:
    ```rust
    #[derive(thiserror::Error, Debug)]
    pub enum FetchReleasesError {
        #[error("failed to fetch releases: {0}")]
        Fetch(#[from] GitHubReleaseFetchError),
    }
    ```

2.  **Serializable Command Errors**:
    -   These errors are defined in the `commands.rs` file for each feature slice. They are the only errors the frontend will ever receive.
    -   They wrap the internal business logic errors using the `#[from]` attribute provided by `thiserror`.
    -   They **must** be serializable. They derive `strum::IntoStaticStr` and implement `serde::Serialize` to format the error into a structured JSON object with a `type` and `message`.

    *Example from `fetch_releases/commands.rs`*:
    ```rust
    #[derive(thiserror::Error, Debug, IntoStaticStr)]
    pub enum FetchReleasesCommandError {
        #[error("system directory not found: {0}")]
        SystemDir(#[from] tauri::Error),

        // This wraps the internal FetchReleasesError
        #[error("failed to fetch releases: {0}")]
        Fetch(#[from] FetchReleasesError),
    }

    // The implementation of `serde::Serialize` follows...
    ```
    This pattern ensures that internal implementation details are not leaked to the frontend, which only receives a clean, structured, and serializable error object.

### Frontend Error Handling

The frontend uses `@tanstack/react-query` to manage API state and a centralized utility function to display toast notifications.

1.  **API State Management**: Components use the `useQuery` hook from `react-query` to call backend commands. This hook automatically provides `data`, `isLoading`, and `error` states.

    *Example from `PlayPage/ReleaseSelector.tsx`*:
    ```tsx
    const {
      data: releases,
      isLoading: isReleasesLoading,
      error: releasesError,
    } = useQuery<GameRelease[]>({
      queryKey: ["releases", variant],
      queryFn: () => fetchReleasesForVariant(variant),
    });
    ```

2.  **Error Handling and Notification**: A `useEffect` hook monitors the `error` object from `useQuery`. If an error exists, it calls the `toastCL` utility to show a user-friendly message. This keeps the error-handling logic separate from the main component rendering.

    *Example from `PlayPage/ReleaseSelector.tsx`*:
    ```tsx
    useEffect(() => {
      if (!releasesError) {
        return;
      }

      toastCL("error", `Failed to fetch releases for ${variant}.`, releasesError);
    }, [releasesError, variant]);
    ```

3.  **Graceful Degradation**: The component uses the loading and error states from `useQuery` to render a responsive UI, such as showing a loading message or disabling elements when an error occurs.

    *Example from `PlayPage/ReleaseSelector.tsx`*:
    ```tsx
    const placeholderText = isReleasesLoading
      ? "Loading..."
      : releasesError
      ? "Error loading releases."
      : "Select a release";

    return (
      <Combobox
        // ...
        placeholder={placeholderText}
        disabled={Boolean(releasesError)}
      />
    );
    ```

This end-to-end system ensures that backend errors are structured, propagated cleanly to the frontend, and handled in a consistent, user-friendly manner.

## ts-rs: TypeScript Type Generation

This project uses the `ts-rs` crate to automatically generate TypeScript type definitions from Rust structs and enums. This ensures that the frontend and backend types are always in sync.

### How it Works

When you run the backend tests, a test case specifically for exporting types is executed. This test will:

1.  Find all Rust types decorated with `#[ts(export)]`.
2.  Generate the corresponding TypeScript types.
3.  Save them to a gitignored directory within the frontend's source tree.

### Agent Responsibility

If you modify any Rust structs or enums that are shared with the frontend (i.e., they have a `#[ts(export)]` attribute), you **must** run the backend tests to regenerate the TypeScript types:

```bash
cargo test --manifest-path ./cat-launcher/src-tauri/Cargo.toml
```

This ensures the frontend has access to the latest type definitions.

### Checking for Syntax Errors

To quickly check your Rust code for syntax errors without compiling, you can run the following command:

```bash
cargo check --manifest-path ./cat-launcher/src-tauri/Cargo.toml
```

## Best Practices

### Prefer Enums Over Booleans

When adding a field to a struct or a payload that represents a state, prefer using an enum over a boolean. For example, instead of `is_finished: bool`, prefer a `status: UpdateStatus` enum with variants like `InProgress` and `Finished`.

### Avoid Methods that May Panic

In Rust, avoid methods that may panic. Instead, use the `Result` type to handle errors gracefully. For example, instead of using `unwrap()` or `expect()` use `if let` or `match` to handle errors.

### Avoid Box<dyn Error>

Instead, define a custom error type using the `thiserror` crate, and use it.

## Agent Workflow

When working on this project, please follow these guidelines:

1.  **Understand the Architecture:** Before making changes, familiarize yourself with the project's architecture as described in this document.
2.  **Maintain Architecture:** When adding new features or modifying existing code, adhere to the established architectural patterns. For example, new backend features should be implemented as new vertical slices, and frontend-backend communication should continue to use the wrappers in `lib/commands.ts`.
3.  **Perform Self-Review:** After making any change, run `cr review --plain -t uncommitted` if the command is available. Address any feedback provided before proceeding.
4.  **Run Tests:** After making any changes, run the relevant tests to ensure that you haven't introduced any regressions.
5.  **Update Documentation:** If you add or modify features, update this `AGENTS.md` file and the `README.md` as needed.
6.  **Follow Existing Conventions:** Adhere to the coding style and conventions already present in the codebase.
