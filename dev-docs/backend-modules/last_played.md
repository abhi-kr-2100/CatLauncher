# Last Played Module

## Motivation

The `last_played` module is a small, focused module responsible for persisting and retrieving the version identifier of the last game release that the user played for each game variant. This is a key piece of state that improves the user experience by allowing the launcher to remember the user's most recent session.

The primary goals of this module are:
1.  **Persistence:** To provide a mechanism for securely storing the last played version string in the application's database.
2.  **Retrieval:** To allow other modules to easily query for the last played version of a given game variant.
3.  **Encapsulation:** To abstract the underlying storage mechanism (the `last_played_versions` table in SQLite) behind a clean, high-level API.

## Design

The module's design is centered around the `LastPlayedVersionRepository` trait, which defines the contract for storing and retrieving the data.

1.  **Core Business Logic (`last_played.rs`):**
    *   The core logic is implemented as methods on the `GameVariant` enum.
    *   **`get_last_played_version`:** This async method takes a reference to a `LastPlayedVersionRepository` and calls its corresponding method to fetch the version string for the given variant. It returns an `Option<String>`, gracefully handling the case where no version has been recorded yet.
    *   **`set_last_played_version`:** This async method takes a version string and a repository, and it calls the repository's method to persist the new value. This method is called by the `launch_game` module just before a game is started.

2.  **Repository (`repository/`):**
    *   **`LastPlayedVersionRepository` Trait:** Defines the abstract interface with `get_last_played_version` and `set_last_played_version` methods.
    *   **`SqliteLastPlayedVersionRepository`:** The concrete implementation of the trait that uses `rusqlite` to interact with the SQLite database. It handles the SQL `INSERT` (with `ON CONFLICT DO UPDATE`) and `SELECT` queries.

3.  **Framework Bridge (`commands.rs`):**
    *   **`get_last_played_version` Command:** Exposes the `get_last_played_version` logic to the frontend. This allows the UI to fetch the last played version when it initializes, for example, to pre-select the correct release in a dropdown menu.

## Workings

1.  **Setting the Version:**
    *   When the user clicks "Play" on a specific release (e.g., version "0.G"), the `launch_game` module is invoked.
    *   Just before launching the game process, `launch_game` calls `variant.set_last_played_version("0.G", &*repository).await`.
    *   The `SqliteLastPlayedVersionRepository` then executes an SQL query to `INSERT OR REPLACE` the entry for that game variant into the `last_played_versions` table.

2.  **Getting the Version:**
    *   When the user opens the application and navigates to the "Play" page, the frontend calls the `get_last_played_version` command.
    *   The command calls `variant.get_last_played_version(&*repository).await`.
    *   The `SqliteLastPlayedVersionRepository` executes a `SELECT` query to find the version string associated with the game variant.
    *   The command returns `Some("0.G")` to the frontend.
    *   The frontend UI can then use this information to automatically select the "0.G" release in the release selector, providing a convenient shortcut for the user.
