# Backend Testing Priority Plan

This document identifies which parts of the CatLauncher backend code need tests the most, prioritizing by **functional criticality** (risk of user data loss, destructive file operations, core launcher workflows) and **existing code coverage deficit** (as measured by [`lcov.info`](file:///home/abhi/Projects/CatLauncher/lcov.info) and [`coverage-html/html/index.html`](file:///home/abhi/Projects/CatLauncher/coverage-html/html/index.html)).

---

## 1. Executive Summary & Assessment Criteria

Testing priority is evaluated along two primary dimensions:

1. **Criticality**:
   * **Data Safety:** Destructive operations (`remove_dir_all`, directory migration, file overwrites).
   * **Core Value Proposition:** Features essential to the launcher (downloading, installing, running games, managing mods and assets).
   * **Operational Complexity:** Database transactions, raw SQL queries, remote API parsing, concurrency, and cross-platform I/O.
2. **Coverage Deficit**:
   * Percentage of uncovered lines, absolute volume of untested code (missed lines), and presence/absence of automated unit or integration tests.

### Current Landscape
* **Strongly Tested (>84% – 99% coverage, 50+ integration tests):** Core release management and gameplay routines are thoroughly tested ([`fetch_releases`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/fetch_releases), [`install_release`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/install_release), [`launch_game`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/launch_game), [`manual_backups`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/manual_backups), [`backups`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/backups), [`master_reset`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/master_reset), and [`game_tips`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/game_tips)).
* **Severely Untested (0% – 18% coverage, 0 tests):** The entire asset management subsystem (mods, soundpacks, tilesets), startup data migration, and play time logging possess **zero tests** despite executing destructive filesystem mutations and raw SQL operations.

---

## 2. Priority Matrix

| Component / Subsystem | Missed Executable Lines | Line Coverage | Existing Tests | Criticality | Primary Risk |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **1. Third-Party Asset Management** ([`mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods), [`soundpacks`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks), [`tilesets`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets)) | 1,470 | ~5.8% | 0 | **Critical** | Corrupted installations, accidental deletions, untested SQL queries |
| **2. Filesystem Migration & Recursive Copy** ([`filesystem/utils.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs) & [`utils.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/utils.rs)) | 167 | ~10% | 0 | **Critical** | Permanent data loss on startup data directory migration |
| **3. Play Time Tracking** ([`play_time`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time)) | 116 | 0.00% | 0 | **Medium-High** | Play session tracking loss, SQLite concurrency bugs |
| **4. Application Self-Updater** ([`infra/autoupdate/update.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/autoupdate/update.rs)) | 48 | 0.00% | 0 | **Medium-High** | Inability to auto-update launcher, silent failures |
| **5. Archive Extraction Edge Branches** ([`infra/archive.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/archive.rs)) | 28 | 76.56% | 0 (for DMG/RAR) | **Medium-High** | Crashes on macOS disk images or RAR mod archives |
| **6. Secondary State Repositories** ([`theme`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/theme), [`users`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/users), [`last_played_world`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/last_played_world)) | 138 | 0.00% | 0 | **Medium-Low** | Preference persistence failure, UI glitched state |

---

## 3. Detailed Subsystem Analysis

### Priority 1: Third-Party Asset Management ([`mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods), [`soundpacks`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks), [`tilesets`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets))

This domain is one of CatLauncher's primary value propositions, yet represents the largest untested surface in the backend (**1,470 missed executable lines across 3,500+ lines of code, 0 tests**).

#### A. [`mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods) (3.49% coverage, 801 missed lines, 0 tests)
* **[`SqliteModsRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/repository/sqlite_mods_repository.rs#L15):** 304 lines of raw SQLite operations (transactions, `INSERT ... ON CONFLICT DO UPDATE`, loading/saving GitHub commit activity in polymorphic tables). Completely untested.
* **[`install_third_party_mod`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/install_third_party_mod.rs#L55-L106):** Downloads archives, unpacks into temp directories, resolves relative `modinfo.json` paths via [`get_mod_parent_dir`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/install_third_party_mod.rs#L114-L124), copies contents to user mod directories, cleans up temp directories, and writes to database. Zero tests.
* **[`uninstall_third_party_mod`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/uninstall_third_party_mod.rs#L16-L35):** Recursively deletes filesystem directories and modifies installed database records. Zero tests.
* **[`list_all_mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/list_all_mods.rs#L94-L180):** Multi-source pipeline: loads cached mods, streams progress updates, queries remote repositories ([`BrightNightsModRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/online/bright_nights/repository.rs#L8)), parses remote GitHub manifests ([`manifest.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/online/bright_nights/manifest.rs)), scans local game directories for stock mods ([`get_all_stock_mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/list_all_mods.rs#L37-L64)), merges, deduplicates, and caches to SQLite. Zero tests.

#### B. [`soundpacks`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks) (7.86% coverage, 340 missed lines, 0 tests)
* **[`install_third_party_soundpack`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks/install_third_party_soundpack.rs#L52-L99):** Downloads archives, extracts files, verifies sound metadata (`soundpack.txt` / `sound_info.json`), copies to game sound directory, and records status in SQLite.
* **[`uninstall_third_party_soundpack`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks/uninstall_third_party_soundpack.rs#L16-L36):** Deletes soundpack directories and updates repository state.
* **[`list_all_soundpacks`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks/list_all_soundpacks.rs#L89-L163):** Reads local directories and manifests.

#### C. [`tilesets`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets) (8.10% coverage, 329 missed lines, 0 tests)
* **[`install_third_party_tileset`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets/install_third_party_tileset.rs#L50-L96):** Downloads, extracts, resolves `tileset.txt`, copies tileset assets into game directory, and registers installation.
* **[`uninstall_third_party_tileset`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets/uninstall_third_party_tileset.rs#L16-L35):** Removes tileset directories and deregisters from SQLite.
* **[`list_all_tilesets`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets/list_all_tilesets.rs#L89-L162):** Discovers and parses local tileset configurations.

---

### Priority 2: Filesystem Migration & Recursive Copying ([`filesystem/utils.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs) & [`utils.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/utils.rs))

* **Coverage:** [`filesystem/utils.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs) is **18.42%** covered; [`migrate_to_local_data_dir_impl`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/utils.rs#L118-L143) is **0.00%** covered.
* **Failure Impact: Catastrophic Data Loss.**
  * On launcher startup, [`migrate_to_local_data_dir_impl`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/utils.rs#L118-L143) checks if legacy `app_data_dir` exists. If so, it invokes [`copy_dir_all`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs#L22-L58) to `app_local_data_dir` and subsequently runs `tokio::fs::remove_dir_all(&app_data_dir)`.
  * [`copy_dir_all`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs#L22-L58) has platform-specific implementations: on macOS it invokes an external `ditto` process, while on Linux/Windows it performs recursive asynchronous directory reading and copying.
  * If [`copy_dir_all`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs#L22-L58) fails midway or mishandles symlinks, permissions, or deep directory trees, user saves, backups, and configurations could be destroyed during deletion.

---

### Priority 3: Play Time Tracking Subsystem ([`play_time`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time))

* **Coverage:** **0.00%** across both [`play_time.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time/play_time.rs) and [`sqlite_play_time_repository.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time/sqlite_play_time_repository.rs).
* **Failure Impact: Play History Loss / Concurrency Lockups.**
  * Tracks total gameplay duration by variant and version.
  * Uses blocking runtime handoffs (`task::block_in_place`, `task::spawn_blocking`), duration validations (`duration_in_seconds <= 0`), and SQL aggregation queries (`COALESCE(SUM(...), 0)`, `ON CONFLICT DO UPDATE SET duration_in_seconds = duration_in_seconds + excluded.duration_in_seconds`).
  * Needs integration tests with SQLite connection pools to verify concurrent writes, negative duration rejection, and accurate summation across game variants.

---

### Priority 4: Application Self-Updater ([`infra/autoupdate/update.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/autoupdate/update.rs))

* **Coverage:** **0.00%** (48 missed lines, 0 tests).
* **Failure Impact: Stranded Outdated Releases.**
  * [`run_updater`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/autoupdate/update.rs#L6) handles checking, downloading, and applying Tauri binary updates while emitting [`UpdateStatus`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/autoupdate/update_status.rs) events to the frontend.
  * Requires tests to ensure error states are properly emitted over IPC rather than failing silently.

---

### Priority 5: Multi-Format Archive Extraction Branches ([`infra/archive.rs`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/archive.rs))

* **Coverage:** **76.56%** overall, but branches on lines 84–106 are **0%** covered.
* **Failure Impact: Crashes or Corrupted Installs for Specific Archive Formats.**
  * While standard `.zip` and `.tar.gz` extractions are well-tested, the macOS DMG mounting handler ([`dmg::Attach`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/archive.rs#L86)) and RAR archive extractor ([`unrar::Archive`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/archive.rs#L98)) have zero test coverage.
  * Any user downloading a mod or release compressed in `.rar` or `.dmg` exercises unverified code paths.

---

### Priority 6: Secondary State Repositories ([`theme`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/theme), [`users`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/users), [`last_played_world`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/last_played_world))

* **Coverage:** **0.00%** across all files.
* **Failure Impact: Minor UX Glitches.**
  * [`SqliteThemePreferenceRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/theme/sqlite_theme_preference_repository.rs#L14): Persists light/dark/system theme selection.
  * [`SqliteUsersRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/users/repository/sqlite_users_repository.rs#L11) and [`get_or_create_user_id`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/users/service.rs#L8): Generates and stores anonymous PostHog analytics UUIDs.
  * [`get_last_played_world`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/last_played_world/last_played_world.rs#L23): Reads `lastworld.json` to display the active world.
  * Non-destructive and low complexity; priority is lower than data-altering components.

---

## 4. Test Implementation Recommendations

### Leverage Existing Test Harness Infrastructure
The codebase already provides testing utilities that can be repurposed to test the uncovered modules:
* [`TestDatabase`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/testing/test_database.rs#L12): Spin up ephemeral in-memory SQLite pools loaded with [`schema.sql`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/resources/schemas/schema.sql) for testing [`SqliteModsRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/repository/sqlite_mods_repository.rs#L15), [`SqlitePlayTimeRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time/sqlite_play_time_repository.rs#L13), and [`SqliteUsersRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/users/repository/sqlite_users_repository.rs#L11).
* [`TestZip`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/testing/test_zip.rs#L12): Generate synthetic archive files on the fly to test [`install_third_party_mod`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/install_third_party_mod.rs#L55), [`install_third_party_soundpack`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/soundpacks/install_third_party_soundpack.rs#L52), and [`install_third_party_tileset`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/tilesets/install_third_party_tileset.rs#L50).
* [`MockHttpClient`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/infra/testing/http_client.rs#L13): Provide deterministic HTTP responses for testing [`list_all_mods`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/list_all_mods.rs#L94) and [`BrightNightsModRepository`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/mods/online/bright_nights/repository.rs#L8).

### Immediate Test Suites to Create

1. **`cat-launcher/src-tauri/src/filesystem/utils.rs` tests:**
   * Test [`copy_dir_all`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/filesystem/utils.rs#L22-L58) copying nested directory structures, empty directories, and binary files.
   * Verify source and destination contents match byte-for-byte.
   * Test error behavior on read-only or invalid destinations.

2. **`cat-launcher/src-tauri/src/mods/repository/sqlite_mods_repository.rs` tests:**
   * Test inserting third-party mods and conflict updates (`ON CONFLICT DO UPDATE`).
   * Test recording and fetching GitHub commit activity (`github_commit_mod_activity`).
   * Test listing cached mods filtering by [`GameVariant`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/variants/game_variant.rs#L12).

3. **`cat-launcher/src-tauri/src/mods/install_third_party_mod.rs` & `uninstall_third_party_mod.rs` tests:**
   * Test downloading a synthetic zip with nested directory structures and validating final destination files.
   * Test uninstalling removes the correct mod folder without impacting neighboring mods.

4. **`cat-launcher/src-tauri/src/play_time/sqlite_play_time_repository.rs` tests:**
   * Test logging positive durations and aggregating totals with [`get_play_time_for_variant`](file:///home/abhi/Projects/CatLauncher/cat-launcher/src-tauri/src/play_time/play_time.rs#L6).
   * Verify negative or zero durations return `InvalidDuration` error.
