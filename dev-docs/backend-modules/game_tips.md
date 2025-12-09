# Game Tips Module

## Motivation

The `game_tips` module is responsible for fetching and displaying in-game tips to the user in the launcher UI. These tips are sourced directly from the game's own data files, ensuring they are authentic and relevant to the user's installed game version. This feature enhances the user experience by providing helpful hints and lore, making the launcher more engaging.

The primary goals are:
1.  **Tip Extraction:** To locate and parse the JSON files (`tips.json`, `hints.json`) from within the game's installation directory.
2.  **Relevance:** To intelligently find the correct set of tips, prioritizing the user's last played version to ensure the tips match the version they are currently playing.
3.  **Resilience:** To fail gracefully and return an empty list if no game is installed or if the tip files cannot be found.

## Design

The module's logic is designed to find the most relevant installed game version and extract the tips from it.

1.  **Core Business Logic (`game_tips.rs`):**
    *   **`get_all_tips_for_variant`:** This is the main orchestrator function. It follows a clear priority order to find the tips:
        1.  **Last Played Version First:** It first checks the `last_played_repository` to see which version the user played last. If a last-played version is found, it immediately tries to load tips from that specific version's installation directory by calling `get_tips_from_version`.
        2.  **Fallback to Any Installed Version:** If no last-played version is recorded, it fetches the list of all cached releases from the `releases_repository`. It then iterates through them, checking the `GameReleaseStatus` of each one. The first release it finds that is `ReadyToPlay` is then used to load the tips from.
        3.  **Empty Result:** If no installed version is found, it returns an empty `Vec<String>`.
    *   **`get_tips_from_version`:** This private helper function performs the actual file I/O.
        1.  It calls `filesystem::paths::get_tip_file_paths` to get the exact, OS-correct paths to the `tips.json` and `hints.json` files within a specific version's installation directory.
        2.  It reads each file, deserializes the JSON content into a `Vec<Tip>`, and flattens the nested text into a single `Vec<String>`.
        3.  It handles cases where a file might not exist or be empty.

2.  **Framework Bridge (`commands.rs`):**
    *   **`get_tips` Command:** A single Tauri command that exposes the tip-fetching logic to the frontend.
    *   It gathers dependencies like the app's data directory and the current OS.
    *   It calls the core `get_all_tips_for_variant` function and returns the resulting list of tips, or an error if the process fails.

## Workings

1.  The frontend, likely on the "Play" page, invokes the `get_tips` command for the currently selected game variant.
2.  The command calls `get_all_tips_for_variant`.
3.  The core logic first queries the `last_played_repository`. If it finds that the user last played version "0.G," it proceeds to the next step with that version.
4.  It calls `get_tips_from_version("0.G", ...)`.
5.  This function, in turn, asks the `filesystem` module for the paths to the tip files for version "0.G."
6.  The `filesystem` module returns the correct paths, for example:
    *   `.../data/Assets/cdda/0.G/data/core/tips.json`
    *   `.../data/Assets/cdda/0.G/data/json/npcs/hints.json`
7.  `get_tips_from_version` reads these files, parses the JSON, and returns a consolidated list of strings.
8.  The command receives this list and sends it back to the frontend, which can then display a random tip from the list.
