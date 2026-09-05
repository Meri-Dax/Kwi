use diesel_async::pooled_connection;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Not found")]
    NotFound,
    #[error("{0}")]
    Query(diesel::result::Error),
    #[error("{0}")]
    Database(#[from] pooled_connection::bb8::RunError),
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => RepositoryError::NotFound,
            err => RepositoryError::Query(err),
        }
    }
}

#[macro_export]
macro_rules! impl_insert {
    ($model:ty, $form:ty, $table:expr) => {
        pub async fn insert(
            AppState { database }: &AppState,
            payload: &$form,
        ) -> Result<$model, $crate::common::repository::RepositoryError> {
            let mut conn = database.get().await?;

            let result = diesel::insert_into($table)
                .values(&payload)
                .returning(<$model>::as_returning())
                .get_result(&mut conn)
                .await?;

            Ok(result)
        }
    };
}
