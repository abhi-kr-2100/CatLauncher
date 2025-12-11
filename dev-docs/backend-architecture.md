# Backend Architecture

## Directory and Path Handling

To maintain a clean separation of concerns and enhance portability, business logic functions should not be tightly coupled to the application's directory structure.

-   **Parameter Passing:** Business logic functions should only receive the top-level directory paths they need as parameters. These are typically:
    -   `cache_dir`: The path to the application's cache directory.
    -   `data_dir`: The path to the application's data directory.
    -   `resources_dir`: The path to the application's resources directory.
-   **Path Construction:** Inside the business logic functions, use the helper functions provided in `src-tauri/src/filesystem/paths.rs` to construct the exact paths to specific files or subdirectories.

This approach ensures that the core logic is not cluttered with path manipulation and can be easily tested by passing in mock directory paths.

### Checking for Syntax Errors

To quickly check your Rust code for syntax errors without compiling, you can run the following command:

```bash
cargo check --manifest-path ./cat-launcher/src-tauri/Cargo.toml
```
