use std::str::FromStr;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::mods::models::ThirdPartyModStatus;
use crate::mods::repository::installed_mods_repository::{
    InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteInstalledModsRepository {
    pool: Pool,
}

impl SqliteInstalledModsRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstalledModsRepository for SqliteInstalledModsRepository {
    async fn list_installed_mods(
        &self,
        variant: &GameVariant,
    ) -> Result<Vec<ThirdPartyModStatus>, InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let variant = variant.to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::List(Box::new(e)))?;
            let mut stmt = conn
                .prepare(
                    "SELECT variant, mod_id, installed_at, last_updated_time FROM installed_mods WHERE variant = ?1 ORDER BY installed_at DESC, mod_id ASC",
                )
                .map_err(|e| InstalledModsRepositoryError::List(Box::new(e)))?;
            let statuses = stmt
                .query_map(rusqlite::params![variant], |row| {
                    let variant_value: String = row.get(0)?;
                    let variant = GameVariant::from_str(&variant_value).map_err(|err| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(err),
                        )
                    })?;
                    let installed_at: i64 = row.get(2)?;

                    Ok(ThirdPartyModStatus {
                        variant,
                        mod_id: row.get(1)?,
                        installed_at: installed_at as u64,
                        last_updated_time: row.get(3)?,
                    })
                })
                .map_err(|e| InstalledModsRepositoryError::List(Box::new(e)))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| InstalledModsRepositoryError::List(Box::new(e)))?;

            Ok(statuses)
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::List(Box::new(e)))?
    }

    async fn get_installed_mod(
        &self,
        variant: &GameVariant,
        mod_id: &str,
    ) -> Result<Option<ThirdPartyModStatus>, InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let variant = variant.to_string();
        let mod_id = mod_id.to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn
                .prepare(
                    "SELECT variant, mod_id, installed_at, last_updated_time FROM installed_mods WHERE variant = ?1 AND mod_id = ?2",
                )
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?;

            let result = stmt.query_row(rusqlite::params![variant, mod_id], |row| {
                let variant_value: String = row.get(0)?;
                let variant = GameVariant::from_str(&variant_value).map_err(|err| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(err),
                    )
                })?;
                let installed_at: i64 = row.get(2)?;

                Ok(ThirdPartyModStatus {
                    variant,
                    mod_id: row.get(1)?,
                    installed_at: installed_at as u64,
                    last_updated_time: row.get(3)?,
                })
            });

            match result {
                Ok(status) => Ok(Some(status)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(err) => Err(InstalledModsRepositoryError::Get(Box::new(err))),
            }
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?
    }

    async fn upsert_installed_mod(
        &self,
        status: &ThirdPartyModStatus,
    ) -> Result<(), InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let status = status.clone();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Upsert(Box::new(e)))?;
            conn.execute(
                "INSERT INTO installed_mods (variant, mod_id, installed_at, last_updated_time) VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(variant, mod_id) DO UPDATE SET installed_at = excluded.installed_at, last_updated_time = excluded.last_updated_time",
                rusqlite::params![
                    status.variant.to_string(),
                    status.mod_id,
                    status.installed_at as i64,
                    status.last_updated_time
                ],
            )
            .map_err(|e| InstalledModsRepositoryError::Upsert(Box::new(e)))?;

            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Upsert(Box::new(e)))?
    }

    async fn delete_installed_mod(
        &self,
        variant: &GameVariant,
        mod_id: &str,
    ) -> Result<(), InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let variant = variant.to_string();
        let mod_id = mod_id.to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?;
            conn.execute(
                "DELETE FROM installed_mods WHERE variant = ?1 AND mod_id = ?2",
                rusqlite::params![variant, mod_id],
            )
            .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_repository() -> SqliteInstalledModsRepository {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();

        {
            let conn = pool.get().unwrap();
            conn.execute_batch(
                "PRAGMA foreign_keys = ON;
                CREATE TABLE variants (name TEXT PRIMARY KEY);
                INSERT INTO variants (name) VALUES ('DarkDaysAhead'), ('BrightNights'), ('TheLastGeneration');
                CREATE TABLE installed_mods (
                    variant TEXT NOT NULL,
                    mod_id TEXT NOT NULL,
                    installed_at INTEGER NOT NULL,
                    last_updated_time TEXT,
                    PRIMARY KEY (variant, mod_id),
                    FOREIGN KEY (variant) REFERENCES variants (name) ON DELETE CASCADE
                );",
            )
            .unwrap();
        }

        SqliteInstalledModsRepository::new(pool)
    }

    #[tokio::test]
    async fn upsert_and_get_installed_mod() {
        let repo = setup_repository();
        let status = ThirdPartyModStatus {
            variant: GameVariant::DarkDaysAhead,
            mod_id: "dda-aftershock".to_string(),
            installed_at: 1_726_000_000,
            last_updated_time: Some("2024-05-10T12:00:00Z".to_string()),
        };

        repo.upsert_installed_mod(&status).await.unwrap();

        let fetched = repo
            .get_installed_mod(&GameVariant::DarkDaysAhead, "dda-aftershock")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(fetched.mod_id, status.mod_id);
        assert_eq!(fetched.installed_at, status.installed_at);
        assert_eq!(fetched.last_updated_time, status.last_updated_time);
    }

    #[tokio::test]
    async fn list_returns_entries_ordered_by_install_time() {
        let repo = setup_repository();

        let older = ThirdPartyModStatus {
            variant: GameVariant::BrightNights,
            mod_id: "bn-dark-skies".to_string(),
            installed_at: 100,
            last_updated_time: None,
        };
        let newer = ThirdPartyModStatus {
            variant: GameVariant::BrightNights,
            mod_id: "bn-secronom".to_string(),
            installed_at: 200,
            last_updated_time: Some("2024-10-01T00:00:00Z".to_string()),
        };

        repo.upsert_installed_mod(&older).await.unwrap();
        repo.upsert_installed_mod(&newer).await.unwrap();

        let results = repo
            .list_installed_mods(&GameVariant::BrightNights)
            .await
            .unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].mod_id, newer.mod_id);
        assert_eq!(results[1].mod_id, older.mod_id);
    }

    #[tokio::test]
    async fn delete_removes_entry() {
        let repo = setup_repository();
        let status = ThirdPartyModStatus {
            variant: GameVariant::TheLastGeneration,
            mod_id: "tlg-solstice".to_string(),
            installed_at: 555,
            last_updated_time: None,
        };

        repo.upsert_installed_mod(&status).await.unwrap();
        repo.delete_installed_mod(&status.variant, &status.mod_id)
            .await
            .unwrap();

        let fetched = repo
            .get_installed_mod(&status.variant, &status.mod_id)
            .await
            .unwrap();

        assert!(fetched.is_none());
    }
}
