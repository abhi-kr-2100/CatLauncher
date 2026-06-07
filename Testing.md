# Testing documentation for `launch_game`

This document details the test suite implemented for the game launch process in `launch_game.rs`.

## Test Scenarios

The following scenarios are covered by the test suite:

### `prepare_launch`
- **`test_prepare_launch_success`**: Verifies that the function correctly sets up the launch command, identifies the dummy executable, and creates a backup entry in the database.
- **`test_prepare_launch_with_world`**: Verifies that the `--world` argument is correctly added to the command when specified.
- **`test_prepare_launch_executable_missing`**: Verifies that an error is returned if the game executable cannot be found in the installation directory.

### `run_game_and_monitor`
- **`test_run_game_and_monitor_success`**: Verifies that logs from the game's `stdout` are captured and emitted as `GameEvent::Log` events, and that the process exit is captured.
- **`test_run_game_and_monitor_stderr`**: Verifies that logs from the game's `stderr` are also captured and emitted.
- **`test_run_game_and_monitor_exit_code`**: Verifies that custom exit codes from the game process are correctly reported in the `GameEvent::Exit` payload.

### `cleanup_old_backups`
- **`test_cleanup_old_backups`**: Verifies that when the number of backups exceeds `MAX_BACKUPS`, the oldest ones are removed from both the database and the filesystem.
- **`test_cleanup_backups_under_limit`**: Verifies that no backups are deleted if the count is within the allowed limit.

### Integration Tests (`launch_and_monitor_game`)
- **`test_full_launch_flow`**: Covers the entire end-to-end process:
  1. Mocks a GitHub release and asset using `github-mock-api`.
  2. Fetches releases to populate the local repository.
  3. Uses `install_release` to download (from the mock server) and extract a dummy asset fixture.
  4. Launches the game and monitors it.
  5. Verifies that the active release is updated in the database.
- **`test_launch_and_monitor_game_release_not_found`**: Verifies that launching fails gracefully if the requested release ID does not exist.

## Test Fixtures

A lightweight dummy release archive `cat-launcher/src-tauri/src/infra/testing/fixtures/dummy-release.tar.gz` was created. It contains a minimal game directory structure:
- `cataclysm-2026-06-07/cataclysm-launcher` (executable)
- `cataclysm-2026-06-07/save/` (directory)
- `cataclysm-2026-06-07/save/dummy_save.sav` (file)

Using this fixture avoids the need to download large real game releases during testing, making the test suite faster and more reliable.

## `github-mock-api` Limitations

During the implementation, several limitations and quirks of the `github-mock-api` crate were encountered:

1.  **Visibility of `ReleaseAsset`**: The `ReleaseAsset` struct is defined in `github-mock-api` but not exported from the crate root. This prevents direct construction of the `assets` field in the `Release` struct in test code.
    - *Workaround*: Used `serde_json` to manually construct a JSON representation of the `Release` (including assets) and then deserialized it into a `MockRelease` struct.
2.  **Asset Download URL Pattern**: The mock server's asset download route follows a strict pattern: `/{owner}/{repo}/releases/download/{tag}/{filename}`. For the `Downloader` to successfully fetch assets from the mock server, the `browser_download_url` in the mocked release metadata must match this pattern and include the mock server's base URI.
3.  **Case Sensitivity**: The `tag` and `filename` components of the download URL are case-sensitive in the mock server's route matching, whereas `owner` and `repo` are lowercased.
4.  **Asset IDs**: While `Release` IDs can be manually set, `Asset` IDs are typically generated or need careful matching between the metadata and the registration call.
