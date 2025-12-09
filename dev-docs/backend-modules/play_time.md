# Play Time Module

## Motivation

The `play_time` module is responsible for tracking, storing, and retrieving the amount of time a user spends playing each game version. This data provides valuable feedback to the user and can be used to display interesting statistics in the launcher UI, such as the total time played for a specific variant or the time played on a particular release.

The primary goals of this module are:
1.  **Time Logging:** To provide a simple interface for logging a play session's duration against a specific game variant and version.
2.  **Data Aggregation:** To calculate and retrieve aggregated play time statistics, such as the total time for a variant (summed across all its versions) or the total time for a specific version.
3.  **Persistence:** To ensure that play time data is saved reliably in the application's database.

## Design

The module follows the standard architectural pattern of separating business logic, data access, and the framework bridge. Its design is centered on the `PlayTimeRepository` trait.

1.  **Core Business Logic (`play_time.rs`):**
    *   This file contains simple, stateless functions that act as a clean interface over the repository.
    *   **`get_play_time_for_variant`:** Takes a game variant and a repository, and calls the corresponding method on the repository to fetch the aggregated play time.
    *   **`get_play_time_for_version`:** Takes a game variant, a version string, and a repository, and calls the repository to fetch the play time for that specific version.
    *   The business logic itself is minimal; the main responsibility of these functions is to delegate to the data access layer.

2.  **Repository (`repository.rs`, `sqlite_play_time_repository.rs`):**
    *   **`PlayTimeRepository` Trait:** Defines the abstract contract for all play time data operations. This includes methods like:
        *   `log_play_time`: Adds a new play time entry.
        *   `get_play_time_for_version`: Gets the total time for a specific version.
        *   `get_play_time_for_variant`: Gets the total time for a game variant.
        *   `get_total_play_time`: Gets the total time across all games.
    *   **`SqlitePlayTimeRepository`:** The concrete implementation of the trait. It handles the raw SQL queries needed to interact with the `play_time` table in the SQLite database. For example, `get_play_time_for_variant` executes a `SELECT SUM(duration_in_seconds) ... GROUP BY game_variant` query.

3.  **Framework Bridge (`commands.rs`):**
    *   **`get_play_time_for_variant` Command:** Exposes the feature of getting the total play time for a variant to the frontend.
    *   **`get_play_time_for_version` Command:** Exposes the feature of getting the play time for a specific version to the frontend.
    *   These commands are thin wrappers that call the corresponding functions in `play_time.rs`, passing in the `SqlitePlayTimeRepository` from Tauri's managed state.

## Workings

1.  **Logging Play Time:**
    *   When a user finishes a game session, the `launch_game` module calculates the duration of the session.
    *   It then calls `play_time_repository.log_play_time(...)`, passing the game variant, version, and the duration in seconds.
    *   The `SqlitePlayTimeRepository` executes an `INSERT` statement to add a new row to the `play_time` table, recording this session.

2.  **Retrieving Play Time:**
    *   The frontend UI, wanting to display the total play time for the "Dark Days Ahead" variant, invokes the `get_play_time_for_variant` command.
    *   The command calls the `get_play_time_for_variant_feature` function.
    *   This function, in turn, calls `repository.get_play_time_for_variant(...)`.
    *   The `SqlitePlayTimeRepository` runs the SQL query: `SELECT SUM(duration_in_seconds) FROM play_time WHERE game_variant = 'DarkDaysAhead'`.
    *   The aggregated result (e.g., `36000` seconds) is returned up the call stack to the frontend, which can then format it for display (e.g., "10 hours").
