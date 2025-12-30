-- Add migration script here
CREATE TABLE
    IF NOT EXISTS settings (
        id INTEGER PRIMARY KEY NOT NULL,
        max_backups INTEGER NOT NULL DEFAULT 5,
        parallel_requests INTEGER NOT NULL DEFAULT 4
    );

INSERT INTO
    settings (id, max_backups, parallel_requests)
VALUES
    (1, 5, 4) ON CONFLICT (id) DO NOTHING;
