use crate::{
    common::repository::RepositoryError,
    entities::course::model::{RecipeCourse, RecipeCourseForm},
    helpers::AppState,
    impl_insert,
    schema::recipe_course,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

impl_insert!(RecipeCourse, RecipeCourseForm, crate::schema::recipe_course::table);

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<RecipeCourse, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = recipe_course::dsl::recipe_course
        .select(RecipeCourse::as_returning())
        .filter(recipe_course::dsl::id.eq(search_id))
        .first(&mut conn)
        .await?;

    Ok(result)
}

pub async fn list(app_state: &AppState) -> Result<Vec<RecipeCourse>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = recipe_course::dsl::recipe_course
        .select(RecipeCourse::as_returning())
        .load(&mut conn)
        .await?;

    Ok(result)
}
