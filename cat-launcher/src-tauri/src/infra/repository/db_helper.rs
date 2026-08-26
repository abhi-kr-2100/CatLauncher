use std::error::Error;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

pub async fn run_db<F, R, E>(
  pool: Pool<SqliteConnectionManager>,
  f: F,
) -> Result<R, Box<dyn Error + Send + Sync>>
where
  F: FnOnce(
      r2d2::PooledConnection<SqliteConnectionManager>,
    ) -> Result<R, E>
    + Send
    + 'static,
  R: Send + 'static,
  E: Error + Send + Sync + 'static,
{
  task::spawn_blocking(move || {
    let conn = pool.get()?;
    f(conn).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
  })
  .await
  .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}
