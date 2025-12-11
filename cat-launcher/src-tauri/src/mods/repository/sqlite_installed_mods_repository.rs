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
                    "SELECT variant, mod_id, installed_at FROM installed_mods WHERE variant = ?1 ORDER BY installed_at DESC, mod_id ASC",
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
                    let installed_at: String = row.get(2)?;

                    Ok(ThirdPartyModStatus {
                        variant,
                        mod_id: row.get(1)?,
                        installed_at,
                        last_updated_time: None,
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
                    "SELECT variant, mod_id, installed_at FROM installed_mods WHERE variant = ?1 AND mod_id = ?2",
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
                let installed_at: String = row.get(2)?;

                Ok(ThirdPartyModStatus {
                    variant,
                    mod_id: row.get(1)?,
                    installed_at,
                    last_updated_time: None,
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
                "INSERT INTO installed_mods (variant, mod_id, installed_at) VALUES (?1, ?2, ?3)
                ON CONFLICT(variant, mod_id) DO UPDATE SET installed_at = excluded.installed_at",
                rusqlite::params![
                    status.variant.to_string(),
                    status.mod_id,
                    status.installed_at
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
