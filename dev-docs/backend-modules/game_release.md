# Game Release Module

## Motivation

The `game_release` module is a foundational data modeling module. Its primary purpose is to define the `GameRelease` struct, which is the central data structure representing a single, installable version of the game. This struct and its associated enums are used throughout the application to manage, display, and interact with game versions.

The key goals of this module are:
1.  **Data Definition:** To provide a clear, strongly-typed definition for what constitutes a "game release."
2.  **State Representation:** To define the possible states of a game release, from not being downloaded to being ready to play, through the `GameReleaseStatus` enum.
3.  **Asset Resolution:** To encapsulate the logic for identifying the correct downloadable asset for a given release based on the user's operating system and architecture.

## Design

The module is primarily composed of data structures and logic that operates on them.

1.  **Core Data Structures (`game_release.rs`):**
    *   **`GameRelease` Struct:** This is the main struct, containing all the essential information about a specific game version:
        *   `variant`: The game variant (e.g., `DarkDaysAhead`, `BrightNights`).
        *   `version`: The version identifier string (e.g., "0.G").
        *   `release_type`: An enum (`Stable`, `Experimental`) categorizing the release.
        *   `status`: A `GameReleaseStatus` enum indicating its current state on the user's machine.
        *   `created_at`: The release date.
    *   **`ReleaseType` Enum:** Defines the categories of releases.
    *   **`GameReleaseStatus` Enum:** Defines the possible states of a release in its lifecycle (e.g., `NotDownloaded`, `NotInstalled`, `ReadyToPlay`).

2.  **Asset Resolution Logic:**
    *   **`GameRelease::get_asset`:** This asynchronous method is responsible for finding the correct downloadable `GitHubAsset` for the release.
        1.  It first fetches the list of all available assets for the release from the `releases_repository`.
        2.  It calls `get_platform_asset_substrs` to get a list of identifying substrings for the user's specific OS and architecture (e.g., `x64-windows-tiles`).
        3.  It then iterates through the substrings and searches the asset list to find the first asset whose name contains one of the substrings. This provides a flexible way to match the correct download file.

3.  **Utility Functions (`utils.rs`):**
    *   **`get_release_by_id`:** A helper function that retrieves the full `GameRelease` object for a given `release_id`. It orchestrates fetching the release information from the repository and determining its current installation status.
    *   **`get_platform_asset_substrs`:** Contains the mapping logic between a platform (OS + Arch) and the expected identifying strings in the asset filenames.

## Workings

When another module, like `install_release`, needs to download a game, it starts with a `GameRelease` object. To find the correct download URL, it calls `game_release.get_asset(...)`.

This method then consults the `releases_repository` to get the list of asset files associated with that release on GitHub. It uses the `get_platform_asset_substrs` utility to determine that, for a 64-bit Windows user, it should look for an asset with "x64-windows-tiles" in its name. It finds the matching asset in the list and returns it.

The `install_release` module can then use the `browser_download_url` from the returned `GitHubAsset` to proceed with the download, confident that it has the correct file for the user's system. This keeps the platform-specific asset selection logic cleanly encapsulated within the `game_release` module.
