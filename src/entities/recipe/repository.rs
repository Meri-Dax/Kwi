use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::recipe::model::{Recipe, RecipeForm, RecipeSearchForm},
    helpers::AppState,
    impl_insert,
    schema::recipe::{dsl as recipe_dsl, id, slug},
};

impl_insert!(Recipe, RecipeForm, crate::schema::recipe::table);

pub async fn search_one(app_state: &AppState, search: RecipeSearchForm) -> Result<Recipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = recipe_dsl::recipe.select(Recipe::as_select()).into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(slug.eq(search_slug));
    }

    let result: Recipe = search_query.first(&mut conn).await?;

    Ok(result)
}
