use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::common::repository::RepositoryError;
use crate::entities::ingredient::model::IngredientSearchForm;
use crate::helpers::AppState;
use crate::schema::ingredient::{id, slug};
use crate::{
    entities::ingredient::model::{Ingredient, IngredientForm},
    impl_insert,
    schema::ingredient::dsl as ingredient_dsl,
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
