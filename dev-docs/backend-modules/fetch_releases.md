# Fetch Releases Module

## Motivation

The `fetch_releases` module is responsible for fetching the list of available game releases for a given game variant. A fast and responsive UI is critical for a good user experience. When the user navigates to the "Play" page, they expect to see the list of available releases immediately. However, fetching the latest releases from a remote source like GitHub can be slow.

This module is designed to address this by providing a multi-layered approach to fetching releases, ensuring the frontend is populated with data as quickly as possible while still getting the most up-to-date information.

## Design

The module is designed to deliver release information in stages, progressively enhancing the list shown to the user.

1.  **Core Business Logic (`fetch_releases.rs`):** The main logic resides in the `fetch_releases` method implemented for the `GameVariant` enum.
    *   **Callback Mechanism:** The function is decoupled from the Tauri framework by accepting an `on_releases` callback. This function is called multiple times to send incremental updates to the frontend.
    *   **Staged Data Fetching:**
        1.  **Default & Cached Releases:** It first loads the "default" releases (a small, curated list bundled with the application) and any previously cached releases from the local SQLite database. These are merged and sent immediately to the frontend via the `on_releases` callback with a `status` of `Fetching`. This ensures the UI is populated with a usable list of releases almost instantly.
        2.  **Remote Fetch:** It then initiates an asynchronous request to the GitHub API to fetch the latest 100 releases for the game variant.
        3.  **Final Update:** Once the GitHub request completes, it sends the full, up-to-date list of releases to the frontend with a `status` of `Success`.
        4.  **Cache Update:** Finally, it updates the local cache in the database with the newly fetched releases so that they are available immediately on the next application launch.

2.  **Framework Bridge (`commands.rs`):** This file exposes the core logic as a Tauri command.
    *   `fetch_releases_for_variant`: This command is invoked by the frontend. It retrieves necessary dependencies like the application's resource directory and the database repository. It then calls the core `fetch_releases` function, providing a concrete implementation for the `on_releases` callback that uses `app_handle.emit` to send the `releases-update` event to the frontend.

3.  **Repository (`repository/`):** This directory contains the `ReleasesRepository` trait and its `SqliteReleasesRepository` implementation, abstracting the database logic for caching releases.

## Workings

When the frontend mounts the `PlayPage`, it calls the `fetch_releases_for_variant` command.

1.  The command immediately triggers the `fetch_releases` function.
2.  Within milliseconds, the function loads default and cached releases, and the frontend receives a `releases-update` event. The UI updates to show this initial list, and a "Loading..." indicator is displayed.
3.  In the background, the request to GitHub is in progress.
4.  Once the GitHub data arrives (typically within a few seconds), the frontend receives a second `releases-update` event. The UI updates with the complete list of releases, and the loading indicator disappears.
5.  The local database is updated for future launches.

This design ensures the user is never looking at a blank or unresponsive screen, greatly improving the perceived performance of the application.
