-- This table stores the available game variants, ensuring that the 'game_variant'
-- column in other tables can only contain valid values.
CREATE TABLE IF NOT EXISTS variants (
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

-- This table stores the last played version for each game variant.
CREATE TABLE IF NOT EXISTS last_played_version (
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

-- This index speeds up the query to get old backups for a game variant.
CREATE INDEX IF NOT EXISTS idx_backups_game_variant_timestamp ON backups (game_variant, timestamp);
