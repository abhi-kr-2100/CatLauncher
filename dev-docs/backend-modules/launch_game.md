# Launch Game Module

## Motivation

The `launch_game` module is responsible for preparing, executing, and monitoring the game process. Launching the game is more than just running an executable; it involves a series of critical side effects that enhance the user's experience and ensure data integrity.

The key responsibilities of this module are:
1.  **Automatic Backups:** To automatically create a backup of the user's save files before every game session, protecting them from data loss or corruption.
2.  **State Management:** To update the "last played" version and track the user's play time.
3.  **Real-time Feedback:** To stream the game's console output (logs, errors) to the launcher's UI in real-time.
4.  **Process Management:** To handle the game process and report its exit status.
5.  **System Hygiene:** To automatically clean up old backups to manage disk space.

## Design

The module is designed to be highly asynchronous to prevent the main application from freezing while the game is running. It separates the preparation logic from the execution and monitoring logic.

1.  **Core Business Logic (`launch_game.rs`):**
    *   **`GameRelease::prepare_launch`:** This function handles all the synchronous setup required before the game can run.
        1.  It locates the game executable for the correct version and OS.
        2.  It updates the database to mark the current version as the "last played."
        3.  It triggers the backup process for the user's save files, creating a new backup entry in the database and a corresponding archive on disk.
        4.  It constructs the `tokio::process::Command`, setting the correct working directory and command-line arguments (most importantly, `--userdir` to point the game to the correct data folder). It configures the command to pipe `stdout` and `stderr` so they can be monitored.
    *   **`run_game_and_monitor`:** This function takes a prepared `Command`, spawns it as a child process, and monitors it.
        1.  It captures the `stdout` and `stderr` streams of the child process.
        2.  It spawns separate Tokio tasks to read the output from each stream line-by-line. Each line is sent back to the caller via the `on_game_event` callback as a `GameEvent::Log`.
        3.  It waits for the child process to exit and then sends a `GameEvent::Exit` event with the exit code.
    *   **`cleanup_old_backups`:** This function runs in the background. It fetches the list of all backups, and if the count exceeds the `max_backups` setting, it deletes the oldest ones from both the database and the filesystem.
    *   **`launch_and_monitor_game`:** This is the main orchestrator. It is a non-blocking function that spawns background tasks to handle the entire game lifecycle.
        1.  It calls `prepare_launch` to get the executable command.
        2.  It spawns a detached Tokio task to run `cleanup_old_backups`.
        3.  It spawns another detached Tokio task that:
            *   Starts a timer to track play time.
            *   Calls `run_game_and_monitor` to execute the game.
            *   Once the game exits, stops the timer and logs the play time to the database.
            *   Reports any errors during the process via the `on_game_event` callback.

2.  **Framework Bridge (`commands.rs`):**
    *   **`launch_game` Command:** The single Tauri command that the frontend invokes.
    *   It gathers all necessary dependencies from Tauri's state.
    *   It creates the `on_game_event` async closure, which uses `app_handle.emit` to forward any `GameEvent` (Log, Exit, Error) to the frontend.
    *   It calls the core `launch_and_monitor_game` function. The command returns `Ok(())` almost immediately, while the game continues to run in the background, communicating via the events.

## Workings

1.  The user clicks "Play" in the UI, which calls the `launch_game` command.
2.  The command collects dependencies and calls `launch_and_monitor_game`. The command itself finishes, unblocking the UI thread.
3.  In the background, `launch_and_monitor_game` prepares and then spawns the game process.
4.  As the game runs, the `run_game_and_monitor` function captures its console output and the `on_game_event` closure emits it to the frontend as `game-event` events. The frontend UI displays these logs in a console view.
5.  Simultaneously, the `cleanup_old_backups` task runs to prune old backups.
6.  When the user closes the game, the `run_game_and_monitor` function detects the process exit and emits a final `game-event` with the exit code. The play time is also logged.
