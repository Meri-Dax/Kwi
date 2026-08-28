use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, PoolError, bb8::Pool},
};
use tracing::info;

use crate::CONFIG;

pub type Database = Pool<AsyncPgConnection>;

pub async fn init() -> Result<Database, PoolError> {
    info!("Initializing database connection pool");

    let config = CONFIG.get().unwrap_or_else(|| unreachable!("This should not happen"));

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);

    let pool = Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .await?;

    Ok(pool)
}
