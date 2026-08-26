use diesel_async::pooled_connection;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Query(#[from] diesel::result::Error),
    #[error("{0}")]
    Database(#[from] pooled_connection::bb8::RunError),
}

#[macro_export]
macro_rules! impl_insert {
    ($model:ty, $form:ty, $table:expr) => {
        pub async fn insert(
            AppState { database }: &AppState,
            payload: $form,
        ) -> Result<$model, $crate::common::repository::Error> {
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
