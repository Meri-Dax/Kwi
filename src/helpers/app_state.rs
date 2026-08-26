use diesel_async::pooled_connection::PoolError;

use crate::helpers::{Database, database};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub async fn try_init() -> Result<Self, PoolError> {
        let database = database::init().await?;

        Ok(Self { database })
    }
}
