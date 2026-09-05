use crate::{
    common::repository::RepositoryError,
    entities::logistics::model::{RecipeLogistics, RecipeLogisticsForm},
    helpers::AppState,
    impl_insert,
    schema::recipe_logistics,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

impl_insert!(
    RecipeLogistics,
    RecipeLogisticsForm,
    crate::schema::recipe_logistics::table
);

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<RecipeLogistics, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = recipe_logistics::dsl::recipe_logistics
        .select(RecipeLogistics::as_returning())
        .filter(recipe_logistics::dsl::id.eq(search_id))
        .first(&mut conn)
        .await?;

    Ok(result)
}

pub async fn list(app_state: &AppState) -> Result<Vec<RecipeLogistics>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = recipe_logistics::dsl::recipe_logistics
        .select(RecipeLogistics::as_returning())
        .load(&mut conn)
        .await?;

    Ok(result)
}
