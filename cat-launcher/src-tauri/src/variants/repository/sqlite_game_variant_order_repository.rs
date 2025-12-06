use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use std::str::FromStr;
use tokio::task;

use crate::variants::repository::game_variant_order_repository::{
    GameVariantOrderRepository, GameVariantOrderRepositoryError, GetGameVariantOrderError,
    UpdateGameVariantOrderError,
};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteGameVariantOrderRepository {
    pool: Pool,
}

impl SqliteGameVariantOrderRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GameVariantOrderRepository for SqliteGameVariantOrderRepository {
    async fn get_ordered_variants(
        &self,
    ) -> Result<Vec<GameVariant>, GameVariantOrderRepositoryError> {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| GetGameVariantOrderError::Get(Box::new(e)))?;
            let mut stmt = conn
                .prepare("SELECT game_variant FROM game_variant_order ORDER BY sort_order")
                .map_err(|e| GetGameVariantOrderError::Get(Box::new(e)))?;
            let rows = stmt
                .query_map([], |row| {
                    let variant_str: String = row.get(0)?;
                    GameVariant::from_str(&variant_str).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })
                })
                .map_err(|e| GetGameVariantOrderError::Get(Box::new(e)))?;

            let mut variants = Vec::new();
            for row in rows {
                variants.push(row.map_err(|e| GetGameVariantOrderError::Get(Box::new(e)))?);
            }

            Ok(variants)
        })
        .await
        .map_err(|e| GetGameVariantOrderError::Get(Box::new(e)))?
        .map_err(GameVariantOrderRepositoryError::Get)
    }

    async fn update_order(
        &self,
        variants: &[GameVariant],
    ) -> Result<(), GameVariantOrderRepositoryError> {
        let pool = self.pool.clone();
        let variants = variants.to_vec();

        task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))?;
            let tx = conn
                .transaction()
                .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))?;

            tx.execute("DELETE FROM game_variant_order", [])
                .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))?;

            for (i, variant) in variants.iter().enumerate() {
                tx.execute(
                    "INSERT INTO game_variant_order (game_variant, sort_order) VALUES (?1, ?2)",
                    rusqlite::params![variant.to_string(), i],
                )
                .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))?;
            }

            tx.commit()
                .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))
        })
        .await
        .map_err(|e| UpdateGameVariantOrderError::Update(Box::new(e)))?
        .map_err(GameVariantOrderRepositoryError::Update)
    }
}
