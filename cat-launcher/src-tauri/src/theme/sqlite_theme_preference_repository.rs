use std::str::FromStr;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;
use tokio::task;

use crate::theme::theme::{Theme, ThemePreference};
use crate::theme::theme_preference_repository::{
    ThemePreferenceRepository, ThemePreferenceRepositoryError,
};

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteThemePreferenceRepository {
    pool: Pool,
}

impl SqliteThemePreferenceRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ThemePreferenceRepository for SqliteThemePreferenceRepository {
    async fn get_preferred_theme(&self) -> Result<ThemePreference, ThemePreferenceRepositoryError> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| ThemePreferenceRepositoryError::Get(Box::new(e)))?;

            let stored_theme: Option<String> = conn
                .query_row(
                    "SELECT theme FROM theme_preferences WHERE _id = 1",
                    [],
                    |row| row.get(0),
                )
                .optional()
                .map_err(|e| ThemePreferenceRepositoryError::Get(Box::new(e)))?;

            match stored_theme {
                Some(theme_value) => {
                    let theme = Theme::from_str(&theme_value).map_err(|_| {
                        ThemePreferenceRepositoryError::InvalidTheme(theme_value.clone())
                    })?;
                    Ok(ThemePreference { theme })
                }
                None => {
                    let default = Theme::Light;
                    conn.execute(
                        "INSERT OR REPLACE INTO theme_preferences (_id, theme) VALUES (1, ?1)",
                        [&default.to_string()],
                    )
                    .map_err(|e| ThemePreferenceRepositoryError::Update(Box::new(e)))?;
                    Ok(ThemePreference { theme: default })
                }
            }
        })
        .await
        .map_err(|e| ThemePreferenceRepositoryError::Get(Box::new(e)))?
    }

    async fn set_preferred_theme(
        &self,
        theme: &Theme,
    ) -> Result<(), ThemePreferenceRepositoryError> {
        let pool = self.pool.clone();
        let theme_value = theme.to_string();
        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| ThemePreferenceRepositoryError::Update(Box::new(e)))?;

            conn.execute(
                "INSERT OR REPLACE INTO theme_preferences (_id, theme) VALUES (1, ?1)",
                [&theme_value],
            )
            .map_err(|e| ThemePreferenceRepositoryError::Update(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| ThemePreferenceRepositoryError::Update(Box::new(e)))?
    }
}
