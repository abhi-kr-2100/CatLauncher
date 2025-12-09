# Settings Module

## Motivation

The `settings` module is responsible for defining and managing the application's user-configurable settings. It provides a strongly-typed data structure that represents the configuration loaded from the `settings.json` file. This module ensures that all configurable parameters, such as the maximum number of backups or the number of parallel downloads, are handled consistently and safely throughout the application.

The primary goals are:
1.  **Data Modeling:** To define a clear, serializable `Settings` struct that maps directly to the `settings.json` file format.
2.  **Type Safety:** To use precise types for settings, such as `NonZeroUsize` for `max_backups`, to enforce valid configuration values at the type level, preventing runtime errors.
3.  **Default Values:** To provide a default configuration so the application can function correctly even if the `settings.json` file is missing or malformed.
4.  **Centralized Access:** To make the application's settings available to all modules via Tauri's managed state.

## Design

The module is very simple and primarily consists of data structures.

1.  **Core Data Structures (`settings.rs`):**
    *   **`Settings` Struct:** This is the main struct that holds all application settings.
        *   `max_backups`: The maximum number of automatic save backups to keep. Using `NonZeroUsize` ensures this value can never be zero, which would be an invalid state.
        *   `parallel_requests`: The number of parallel requests the downloader can make. `NonZeroU16` prevents a value of zero.
        *   `games`: A `HashMap` that can store variant-specific settings, though it is not heavily used in the current implementation.
    *   **`GameSettings` Struct:** A nested struct for game-specific settings.

2.  **Default Implementation:**
    *   The `Settings` struct implements the `Default` trait.
    *   This implementation provides sensible default values for all settings (e.g., `max_backups` defaults to 50). This is crucial because it guarantees that a valid `Settings` object can always be created.

3.  **Loading and State Management (in `main.rs`):**
    *   The actual loading of the `settings.json` file and the creation of the `Settings` object happens in the `main.rs` file.
    *   The `main` function attempts to read `settings.json` from the resource directory. If the file is missing or parsing fails, it falls back to `Settings::default()`.
    *   This loaded `Settings` object is then placed into Tauri's managed state using `tauri::Builder::manage(settings)`. This makes the `Settings` object available as a `tauri::State<Settings>` injectable dependency in any Tauri command.

## Workings

1.  **Application Startup:** When the CatLauncher starts, `main.rs` is executed. It looks for `settings.json`.
    *   **Case A (File Exists):** It reads and deserializes the JSON into the `Settings` struct.
    *   **Case B (File Missing/Invalid):** It logs a warning and creates a `Settings` object using `Settings::default()`.
2.  **State Injection:** The resulting `Settings` object is passed to `tauri::Builder::manage()`.
3.  **Usage in a Command:** A command in another module, like `install_release`, needs to know how many parallel downloads to use. It declares `settings: State<'_, Settings>` in its function signature.
    ```rust
    #[command]
    pub async fn install_release(
        // ...
        settings: State<'_, Settings>,
        // ...
    ) -> Result<GameRelease, InstallReleaseCommandError> {
        // ...
        let parallel_requests = settings.parallel_requests;
        // ...
    }
    ```
4.  Tauri's dependency injection system automatically provides a shared, immutable reference to the managed `Settings` object. The command can then access the configuration values safely, knowing that they will always be valid due to the `Default` implementation and the use of non-zero integer types.
