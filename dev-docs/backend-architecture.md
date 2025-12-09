# Backend Architecture

The backend follows a **Vertical Slice Architecture**. Each feature (e.g., `fetch_releases`, `install_release`) is encapsulated within its own module in `src-tauri/src/`. This modular approach keeps all related logic for a feature self-contained.

Within each slice, the principles of **Clean Architecture** are applied:
- **Framework Agnostic Core:** The core business logic (e.g., in `fetch_releases.rs`) is decoupled from the Tauri framework. It receives dependencies like paths and HTTP clients via arguments (dependency injection), making it easy to test in isolation.
- **Framework Bridge:** The `commands.rs` file in each module is the only part that interacts directly with Tauri. It acts as a bridge, exposing the core logic to the frontend as Tauri commands. The command function itself should not contain any business logic. Instead, it should call a function in the feature file (e.g., `fetch_releases.rs`) that contains the actual logic.

## Directory and Path Handling

To maintain a clean separation of concerns and enhance portability, business logic functions should not be tightly coupled to the application's directory structure.

-   **Parameter Passing:** Business logic functions should only receive the top-level directory paths they need as parameters. These are typically:
    -   `cache_dir`: The path to the application's cache directory.
    -   `data_dir`: The path to the application's data directory.
    -   `resources_dir`: The path to the application's resources directory.
-   **Path Construction:** Inside the business logic functions, use the helper functions provided in `src-tauri/src/filesystem/paths.rs` to construct the exact paths to specific files or subdirectories.

This approach ensures that the core logic is not cluttered with path manipulation and can be easily tested by passing in mock directory paths.

## Error Handling

The project has a standardized approach to error handling to ensure that errors are managed gracefully and provide clear feedback to the user and developers.

The backend employs a two-tier error handling strategy to separate internal business logic errors from the errors exposed to the frontend.

1.  **Internal Business Logic Errors**:
    -   These errors are defined within the core logic files (e.g., `fetch_releases.rs`).
    -   They use `thiserror::Error` for clean error propagation within the backend.
    -   They are **not** intended to be sent to the frontend and therefore do **not** implement `serde::Serialize` or derive `strum::IntoStaticStr`.
    -   Each function has its own error enums.

2.  **Serializable Command Errors**:
    -   These errors are defined in the `commands.rs` file for each feature slice. They are the only errors the frontend will ever receive.
    -   They wrap the internal business logic errors using the `#[from]` attribute provided by `thiserror`.
    -   They **must** be serializable. They derive `strum::IntoStaticStr` and implement `serde::Serialize` to format the error into a structured JSON object with a `type` and `message`.

This pattern ensures that internal implementation details are not leaked to the frontend, which only receives a clean, structured, and serializable error object.

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

## Modules

- [backups](./backend-modules/backups.md)
- [fetch_releases](./backend-modules/fetch_releases.md)
- [filesystem](./backend-modules/filesystem.md)
- [game_release](./backend-modules/game_release.md)
- [game_tips](./backend-modules/game_tips.md)
- [infra](./backend-modules/infra.md)
- [install_release](./backend-modules/install_release.md)
- [last_played](./backend-modules/last_played.md)
- [launch_game](./backend-modules/launch_game.md)
- [play_time](./backend-modules/play_time.md)
- [settings](./backend-modules/settings.md)
- [variants](./backend-modules/variants.md)
