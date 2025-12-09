# Filesystem Module

## Motivation

The `filesystem` module is a core utility module responsible for centralizing all logic related to file and directory paths within the application. A key principle of the project's architecture is to keep business logic decoupled from the filesystem structure. Modules should not construct their own paths; instead, they should ask this module for the correct path to a resource.

The primary goals of this module are:
1.  **Single Source of Truth:** To provide a single, authoritative source for all application paths, from the database file to game installation directories and backups.
2.  **Abstraction and Portability:** To abstract away the complexities of different filesystem layouts, especially differences between operating systems (Windows, macOS, Linux).
3.  **Consistency:** To ensure that all parts of the application access resources from the same location.
4.  **Directory Guarantees:** For functions that return a directory path, the module guarantees that the directory will be created if it does not already exist, preventing I/O errors in other modules.

## Design

The module is composed of a collection of public functions, each responsible for returning a specific, well-defined `PathBuf`.

1.  **Core Logic (`paths.rs`):** This is the main file, containing functions that other modules call to get paths.
    *   **Path Derivation:** Functions take high-level identifiers as arguments (e.g., `GameVariant`, `release_version`, a top-level `data_dir`) and use them to construct precise, low-level file paths.
    *   **Directory Creation:** Functions with `get_or_create_` in their name (e.g., `get_or_create_asset_installation_dir`) use `tokio::fs::create_dir_all` to ensure the directory exists before returning the path. This makes filesystem operations in other modules more robust.
    *   **OS-Specific Logic:** Functions like `get_game_executable_dir` and `get_game_executable_filename` contain `match` statements or `if` conditions to handle the different directory structures and executable names required by Windows, macOS, and Linux. For example, on macOS, the executable is located deep inside the `.app` bundle, and this module correctly resolves that path.
    *   **Filename Safety:** It uses a `get_safe_filename` utility to sanitize release versions before using them as directory names, preventing issues with special characters.

2.  **Utilities (`utils.rs`):** This file contains helper functions, such as `get_safe_filename`, which are used internally by `paths.rs`.

## Workings

When another module, such as `launch_game`, needs to find the game executable, it does not build the path itself. Instead, it calls:
`filesystem::paths::get_game_executable_filepath(&self.variant, &self.version, data_dir, os).await?`

The `get_game_executable_filepath` function then performs a series of steps:
1.  It calls `get_or_create_asset_installation_dir` to get the base directory for that specific game version.
2.  It then calls `get_game_executable_dir`, which contains the OS-specific logic to find the subdirectory containing the executable (e.g., handling the `.app` bundle on macOS).
3.  Finally, it calls `get_game_executable_filename` to get the correct name of the executable for the given OS and game variant.
4.  It joins these parts together to form the final, validated path and returns it.

By centralizing this logic, the `launch_game` module remains clean and focused on its core responsibility of running the game, without needing to know the intricate details of the filesystem layout. This makes the codebase easier to maintain, test, and adapt to future changes.
