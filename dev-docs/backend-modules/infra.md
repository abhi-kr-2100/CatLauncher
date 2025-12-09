# Infrastructure Module

## Motivation

The `infra` module (short for infrastructure) is a foundational, horizontal module that provides low-level, reusable components and utilities for the entire backend. Unlike the vertical slice modules (e.g., `fetch_releases`, `install_release`), `infra` does not represent a specific feature. Instead, it encapsulates the logic for interacting with external systems and performing common, cross-cutting tasks.

The key goals of this module are:
1.  **Abstraction:** To abstract away the implementation details of external services and libraries. For example, the rest of the application doesn't need to know how `reqwest` or `zip` works; it just uses the clients and functions provided by `infra`.
2.  **Centralization:** To provide a single, consistent place for handling external concerns like HTTP requests, GitHub API interaction, and archive manipulation.
3.  **Reusability:** To create components (like the HTTP client) that can be shared and reused by multiple feature modules.
4.  **Configuration:** To manage the global configuration for these components, such as the application's user-agent string.

## Design

The `infra` module is organized into several sub-modules, each responsible for a specific piece of infrastructure.

1.  **`http_client.rs`:**
    *   **Motivation:** Manages the application's global `reqwest::Client`. Using a single, shared client is critical for performance, as it allows for connection pooling and reuse.
    *   **Design:** It defines a `lazy_static` global variable, `HTTP_CLIENT`, which is an instance of `reqwest::Client`. This client is configured with a custom user-agent string to identify the application to external APIs.
    *   **Usage:** Other modules (like `fetch_releases`) use this shared client for all their HTTP requests.

2.  **`github/` submodule:**
    *   **Motivation:** Encapsulates all logic for interacting with the GitHub API. This keeps the specifics of GitHub's REST API out of the feature modules.
    *   **Design:** It contains functions like `fetch_github_releases` which handles the API request, deserializes the JSON response into strongly-typed structs (e.g., `GitHubRelease`), and manages API-specific concerns like pagination (fetching up to 100 releases). It also defines the data structures (`GitHubAsset`, `GitHubRelease`) that map directly to the GitHub API's JSON response.

3.  **`archive.rs`:**
    *   **Motivation:** Provides a simple, high-level interface for handling compressed archives. The application needs to extract `.zip` and `.7z` files, and this module abstracts the underlying libraries.
    *   **Design:** It contains a single primary function, `extract_archive`. This function inspects the file extension of the archive and then delegates to the appropriate library (`zip` or `sevenz-rust`) to perform the extraction. It also handles OS-specific permissions, such as setting the executable bit on Linux and macOS after extraction.

4.  **`utils.rs`:**
    *   **Motivation:** A collection of miscellaneous utility functions that are widely used across the application.
    *   **Design:** Contains helpers like `get_os_enum` and `get_arch_enum` to provide a strongly-typed representation of the current operating system and architecture, and `get_github_repo_for_variant` to map a game variant to its GitHub repository.

5.  **`repository/` submodule:**
    *   **Motivation:** Defines the database connection and migration logic.
    *   **Design:** It provides the `setup_database` function, which establishes the connection to the SQLite database and uses the `rusqlite_migration` crate to apply all necessary schema migrations. This ensures the database is always up-to-date when the application starts.

## Workings

When the `install_release` module needs to download and extract a game, it relies heavily on the `infra` module:
1.  It gets the `GitHubAsset` to download, which was originally fetched by the `infra::github` module.
2.  It uses the global `infra::http_client::HTTP_CLIENT` to create a `downloader` instance to perform the download.
3.  After the download is complete, it calls `infra::archive::extract_archive`, passing the path to the downloaded file.
4.  The `extract_archive` function handles the complexity of choosing the right extraction library and setting file permissions, shielding the `install_release` module from these low-level details.
