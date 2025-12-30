use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri_plugin_path::{app_data_dir, AppHandle};
use thiserror::Error;

use crate::constants::{APP_NAME, DATABASE_FILENAME};

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Could not get application data directory")]
    CouldNotGetAppDataDir,

    #[error("Could not create application data directory")]
    CreateAppDataDir(#[source] std::io::Error),

    #[error("Could not create database")]
    CreateDatabase(#[source] sqlx::Error),

    #[error("Could not connect to the database")]
    Connection(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self, DatabaseError> {
        let app_data_dir =
            app_data_dir(app_handle).ok_or(DatabaseError::CouldNotGetAppDataDir)?;
        let app_data_dir = app_data_dir.join(APP_NAME);

        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir).map_err(DatabaseError::CreateAppDataDir)?;
        }

        let db_path = app_data_dir.join(DATABASE_FILENAME);
        let db_url = db_path.to_str().unwrap();

        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            Sqlite::create_database(db_url)
                .await
                                .map_err(DatabaseError::CreateDatabase)?;
        }

        let pool = SqlitePool::connect(db_url).await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(DatabaseError::Connection)?;

        Ok(Self { pool })
    }
}
