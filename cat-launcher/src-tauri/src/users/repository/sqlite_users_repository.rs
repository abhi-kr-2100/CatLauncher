use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use super::users_repository::{UsersRepository, UsersRepositoryError};

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteUsersRepository {
    pool: Pool,
}

impl SqliteUsersRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for SqliteUsersRepository {
    async fn get_or_create_user(&self, id: &str) -> Result<String, UsersRepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| UsersRepositoryError::Create(Box::new(e)))?;

            conn.execute(
                "INSERT OR IGNORE INTO users (_id, id) VALUES (1, ?1)",
                [&id],
            )
                .map_err(|e| UsersRepositoryError::Create(Box::new(e)))?;

            let mut stmt = conn
                .prepare("SELECT id FROM users LIMIT 1")
                .map_err(|e| UsersRepositoryError::Get(Box::new(e)))?;

            let user_id: String = stmt
                .query_row([], |row| row.get(0))
                .map_err(|e| UsersRepositoryError::Get(Box::new(e)))?;

            Ok(user_id)
        })
        .await
        .map_err(|e| UsersRepositoryError::Create(Box::new(e)))?
    }
}
