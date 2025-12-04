use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use crate::variants::GameVariant;
use std::str::FromStr;

pub struct GameVariantOrderRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl GameVariantOrderRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn get_ordered_variants(&self) -> Result<Vec<GameVariant>> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT game_variant FROM game_variant_order ORDER BY sort_order")?;
        let rows = stmt.query_map([], |row| {
            let variant_str: String = row.get(0)?;
            GameVariant::from_str(&variant_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })
        })?;

        let mut variants = Vec::new();
        for row in rows {
            variants.push(row?);
        }

        Ok(variants)
    }

    pub fn update_order(&self, variants: &[GameVariant]) -> Result<()> {
        let mut conn = self.pool.get().unwrap();
        let tx = conn.transaction()?;

        tx.execute("DELETE FROM game_variant_order", [])?;

        for (i, variant) in variants.iter().enumerate() {
            tx.execute(
                "INSERT INTO game_variant_order (game_variant, sort_order) VALUES (?1, ?2)",
                params![variant.to_string(), i],
            )?;
        }

        tx.commit()
    }
}
