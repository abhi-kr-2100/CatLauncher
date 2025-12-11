# Best Practices

### Prefer Enums Over Booleans

When adding a field to a struct or a payload that represents a state, prefer using an enum over a boolean. For example, instead of `is_finished: bool`, prefer a `status: UpdateStatus` enum with variants like `InProgress` and `Finished`.

### Avoid Methods that May Panic

In Rust, avoid methods that may panic. Instead, use the `Result` type to handle errors gracefully. For example, instead of using `unwrap()` or `expect()` use `if let` or `match` to handle errors.

### Avoid Box<dyn Error>

Instead, define a custom error type using the `thiserror` crate, and use it.

### Avoid `anyhow`

The project has its own way of handling errors using `thiserror`. Do not use `anyhow`.

### Pass Arguments to Mutation

When using `useMutation`, pass arguments to the mutation function, instead of referencing variables that may change and cause a race condition.

### `mod.rs` files

`mod.rs` files should only contain module declarations. They should not contain any code. Code should be in other files.

### Import Order Guidelines

**Rust:**
1. Standard library imports (`std::...`).
2. Third‑party crates.
3. Project‑specific crates outside the core (e.g., `cat-macros`).
4. Internal modules of `cat-launcher`.

**JavaScript/TypeScript:**
1. Third‑party libraries.
2. Project source files.

These conventions help keep imports tidy and consistent across the codebase.

### Error Handling with Strongly-Typed Errors

Error enums should wrap lower-level errors using strongly-typed error variants, not strings. For example:

```rust
// Good: Strongly-typed error wrapping
#[derive(thiserror::Error, Debug)]
pub enum InstallModError {
    #[error("failed to get user game data directory: {0}")]
    UserDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to extract archive: {0}")]
    Extraction(#[from] ExtractionError),
}

// Bad: String-based error wrapping
#[derive(thiserror::Error, Debug)]
pub enum InstallModError {
    #[error("failed to get user game data directory: {0}")]
    UserDataDir(String),  // Don't do this
}
```

### Use Shared Infrastructure Utilities

The project provides shared utilities that should be reused:

-   **HTTP Client:** Use `HTTP_CLIENT` from `src-tauri/src/infra/http_client.rs` instead of creating new `reqwest::Client` instances.
-   **Archive Extraction:** Use `extract_archive` from `src-tauri/src/infra/archive.rs` for extracting zip, tar.gz, dmg, and rar files.
-   **Directory Copying:** Use `copy_dir_all` from `src-tauri/src/filesystem/utils.rs` for recursive directory copying.
