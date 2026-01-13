-- This table stores the available game variants, ensuring that the 'game_variant'
-- column in other tables can only contain valid values.
CREATE TABLE IF NOT EXISTS variants (
    name TEXT PRIMARY KEY
);

-- This table stores the supported UI themes and ensures related tables
-- can only reference valid theme names.
CREATE TABLE IF NOT EXISTS themes (
    name TEXT PRIMARY KEY
);

-- This table stores the GitHub release information for each game variant.
CREATE TABLE IF NOT EXISTS releases (
    id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL,
    prerelease INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    FOREIGN KEY (game_variant) REFERENCES variants (name)
);

-- This table stores release notes for each release.
-- It is separate from the releases table to avoid altering the existing table schema.
CREATE TABLE IF NOT EXISTS release_notes (
    release_id INTEGER PRIMARY KEY,
    body TEXT,
    FOREIGN KEY (release_id) REFERENCES releases (id) ON DELETE CASCADE
);

-- This index speeds up filtering releases by game_variant, which is a common
-- operation in both get_cached_releases and update_cached_releases.
CREATE INDEX IF NOT EXISTS idx_releases_game_variant ON releases (game_variant);

-- This unique index prevents duplicate entries for the same tag_name and game_variant,
-- ensuring data integrity.
CREATE UNIQUE INDEX IF NOT EXISTS idx_releases_tag_name_game_variant ON releases (tag_name, game_variant);

-- This table stores the assets associated with each GitHub release.
CREATE TABLE IF NOT EXISTS assets (
    id INTEGER PRIMARY KEY,
    release_id INTEGER NOT NULL,
    browser_download_url TEXT NOT NULL,
    name TEXT NOT NULL,
    digest TEXT,
    FOREIGN KEY (release_id) REFERENCES releases (id) ON DELETE CASCADE
);

-- This index speeds up the JOIN operation between the 'releases' and 'assets'
-- tables in the get_cached_releases function.
CREATE INDEX IF NOT EXISTS idx_assets_release_id ON assets (release_id);

-- This table stores the active release for each game variant.
CREATE TABLE IF NOT EXISTS active_release (
    game_variant TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    FOREIGN KEY (game_variant) REFERENCES variants (name)
);

-- This table stores metadata for each backup created for a game variant.
CREATE TABLE IF NOT EXISTS backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    game_variant TEXT NOT NULL,
    release_version TEXT NOT NULL,
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

-- This composite index speeds up queries that filter by game_variant and order by timestamp.
CREATE INDEX IF NOT EXISTS idx_backups_game_variant_timestamp ON backups (game_variant, timestamp);

-- This table stores the play time for each game variant and version.
CREATE TABLE IF NOT EXISTS play_time (
    id INTEGER PRIMARY KEY,
    game_variant TEXT NOT NULL,
    version TEXT NOT NULL,
    duration_in_seconds INTEGER NOT NULL CHECK (duration_in_seconds >= 0),
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

-- This unique index prevents duplicate entries for the same game_variant and version,
-- and speeds up filtering play time by game_variant and version.
CREATE UNIQUE INDEX IF NOT EXISTS idx_play_time_game_variant_version ON play_time (game_variant, version);

-- This table stores the order of the game variants in the launcher.
CREATE TABLE IF NOT EXISTS game_variant_order (
    game_variant TEXT PRIMARY KEY,
    sort_order INTEGER NOT NULL,
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

-- This table stores unique user identity.
-- The _id column with CHECK(_id = 1) ensures only one row can exist.
CREATE TABLE IF NOT EXISTS users (
    _id INTEGER PRIMARY KEY DEFAULT 1 CHECK(_id = 1),
    id TEXT NOT NULL UNIQUE
);

-- This table stores the persisted theme preference for the launcher UI.
CREATE TABLE IF NOT EXISTS theme_preferences (
    _id INTEGER PRIMARY KEY DEFAULT 1 CHECK(_id = 1),
    theme TEXT NOT NULL,
    FOREIGN KEY (theme) REFERENCES themes (name)
);

-- This table stores metadata for each manual backup created for a game variant.
CREATE TABLE IF NOT EXISTS manual_backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    game_variant TEXT NOT NULL,
    notes TEXT,
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

-- This composite index speeds up queries that filter by game_variant and order by timestamp.
CREATE INDEX IF NOT EXISTS idx_manual_backups_game_variant_timestamp ON manual_backups (game_variant, timestamp);

-- This table stores installed mods for each game variant.
CREATE TABLE IF NOT EXISTS installed_mods (
    mod_id TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    PRIMARY KEY (mod_id, game_variant),
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_installed_mods_game_variant ON installed_mods (game_variant);

-- This table stores installed tilesets for each game variant.
CREATE TABLE IF NOT EXISTS installed_tilesets (
    tileset_id TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    PRIMARY KEY (tileset_id, game_variant),
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_installed_tilesets_game_variant ON installed_mods (game_variant);

-- This table stores installed soundpacks for each game variant.
CREATE TABLE IF NOT EXISTS installed_soundpacks (
    soundpack_id TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    PRIMARY KEY (soundpack_id, game_variant),
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_installed_soundpacks_game_variant ON installed_soundpacks (game_variant);

-- This table stores third-party mod information (both bundled and online).
CREATE TABLE IF NOT EXISTS mods (
    id TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    download_url TEXT NOT NULL,
    modinfo_path TEXT NOT NULL,
    activity_type TEXT NOT NULL,
    PRIMARY KEY (id, game_variant),
    FOREIGN KEY (game_variant) REFERENCES variants (name) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mods_game_variant ON mods (game_variant);

-- This table stores GitHub commit activity specific data for mods.
CREATE TABLE IF NOT EXISTS github_commit_mod_activity (
    mod_id TEXT NOT NULL,
    game_variant TEXT NOT NULL,
    github_url TEXT NOT NULL,
    PRIMARY KEY (mod_id, game_variant),
    FOREIGN KEY (mod_id, game_variant) REFERENCES mods (id, game_variant) ON DELETE CASCADE
);

-- This table stores application settings.
CREATE TABLE IF NOT EXISTS settings (
    _id INTEGER PRIMARY KEY DEFAULT 1 CHECK(_id = 1),
    font_path TEXT
);

-- This table stores the persisted color theme preference for the launcher.
CREATE TABLE IF NOT EXISTS color_settings (
    _id INTEGER PRIMARY KEY DEFAULT 1 CHECK(_id = 1),
    theme_path TEXT
);