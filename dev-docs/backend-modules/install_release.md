# Install Release Module

## Motivation

The `install_release` module is responsible for the entire process of downloading and installing a game release. This is one of the most complex and long-running operations in the application, involving network requests, large file I/O, and archive extraction.

The primary goals of this module are:
1.  **Robust Installation:** To reliably manage the installation state machine, from downloading to extraction and finalization.
2.  **User Feedback:** To provide clear and continuous feedback to the user about the installation progress, including download percentage and current status (e.g., "Downloading," "Installing").
3.  **Filesystem Management:** To ensure that game files are placed in the correct versioned directories and that old installations are cleaned up to prevent wasted disk space.

## Design

The module's design centers around the `install_release` method on the `GameRelease` struct, which orchestrates the installation state machine.

1.  **Core Business Logic (`install_release.rs`):**
    *   **State Machine:** The `install_release` function acts as a state machine. It checks the current `GameReleaseStatus` and proceeds only if the release is not already `ReadyToPlay`.
    *   **Asset Selection:** It identifies the correct game asset to download based on the user's operating system (OS) and architecture (Arch).
    *   **Downloading:**
        *   It sends a `Downloading` status update via the `on_status_update` callback.
        *   It uses a configurable `downloader` to fetch the asset file, supporting features like parallel requests from the `settings` module.
        *   It accepts a `progress` reporter (an `Arc<dyn Reporter>`) to stream download progress back to the caller.
    *   **Installation:**
        *   After a successful download, it sends an `Installing` status update.
        *   It extracts the downloaded archive into a version-specific installation directory (e.g., `.../data/cat-aclysm/stable/0.G/`).
    *   **Cleanup:** After extraction, it removes the temporary downloaded archive. It then calls `delete_other_installations` to scan the parent directory and remove any other versioned installation folders, ensuring only the newly installed version remains.
    *   **Finalization:** It sends a `Success` status update to signal that the installation is complete and the game is ready to play.

2.  **Framework Bridge (`commands.rs`):**
    *   **`install_release` Command:** This single Tauri command exposes the installation logic to the frontend.
    *   **Dependency Injection:** It is responsible for gathering all necessary dependencies from Tauri's state and `AppHandle` (e.g., data paths, settings, repositories).
    *   **Callback and Reporter Implementation:**
        *   It implements the `on_status_update` callback as an async closure that emits `installation-status-update` events to the frontend.
        *   It creates a `ChannelReporter`, which is a custom implementation of the `Reporter` trait. This reporter bridges the downloader's synchronous progress updates with Tauri's asynchronous event system by sending them over the `on_download_progress` IPC channel.

## Workings

1.  The user clicks the "Install" button in the frontend, which invokes the `install_release` command with the selected `release_id` and a Tauri `Channel` for progress reporting.
2.  The command fetches the full `GameRelease` details and gathers all required dependencies.
3.  It calls the core `install_release` function, passing in two ways for the function to communicate back to the frontend:
    *   An `on_status_update` closure to report changes in the overall status (e.g., from `Downloading` to `Installing`).
    *   The `ChannelReporter` to stream fine-grained download progress (bytes downloaded, total size).
4.  As the core function executes, the frontend receives a series of events:
    *   An `installation-status-update` event with `status: "Downloading"`.
    *   A stream of progress updates on the `on_download_progress` channel.
    *   An `installation-status-update` event with `status: "Installing"`.
    *   An `installation-status-update` event with `status: "Success"`.
5.  The frontend UI listens for these events and updates the progress bar and status text accordingly, providing a seamless and informative experience for the user.
