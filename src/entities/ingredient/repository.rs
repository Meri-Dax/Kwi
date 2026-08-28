use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::ingredient::model::{
        Ingredient, IngredientForm, IngredientSearchForm,
    },
    helpers::AppState,
    impl_insert,
    schema::ingredient::{dsl as ingredient_dsl, id, slug},
};

impl_insert!(Ingredient, IngredientForm, crate::schema::ingredient::table);

pub async fn search_one(
    app_state: &AppState,
    search: IngredientSearchForm,
) -> Result<Ingredient, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = ingredient_dsl::ingredient
        .select(Ingredient::as_select())
        .into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(slug.eq(search_slug));
    }

    let result: Ingredient = search_query.first(&mut conn).await?;

    Ok(result)
}
