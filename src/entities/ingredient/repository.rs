use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::{DietaryRestriction, IngredientDietaryRestriction},
        ingredient::model::{Ingredient, IngredientForm, IngredientSearchForm},
    },
    helpers::AppState,
    impl_insert,
    schema::{dietary_restriction, ingredient, ingredient_dietary_restriction},
};

impl_insert!(Ingredient, IngredientForm, crate::schema::ingredient::table);

pub async fn search_one(app_state: &AppState, search: IngredientSearchForm) -> Result<Ingredient, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = ingredient::dsl::ingredient.select(Ingredient::as_select()).into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(ingredient::dsl::id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(ingredient::dsl::slug.eq(search_slug));
    }

    let result: Ingredient = search_query.first(&mut conn).await?;

    Ok(result)
}

pub async fn insert_with_diet(
    app_state: &AppState,
    ingredient: IngredientForm,
    diet_restriction_list: Vec<uuid::Uuid>,
) -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    let result = tx
        .run(
            async |ts_conn| -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
                let ingredient = diesel::insert_into(ingredient::table)
                    .values(&ingredient)
                    .returning(Ingredient::as_returning())
                    .get_result(ts_conn)
                    .await?;

                let diet_restriction_ids_list = diet_restriction_list.clone();
                let diet_restriction_links: Vec<IngredientDietaryRestriction> = diet_restriction_list
                    .into_iter()
                    .map(|dietary_restriction_id| IngredientDietaryRestriction {
                        dietary_restriction_id,
                        ingredient_id: ingredient.id.clone(),
                    })
                    .collect();

                diesel::insert_into(ingredient_dietary_restriction::table)
                    .values(&diet_restriction_links)
                    .execute(ts_conn)
                    .await?;

                let diet_restriction_list = dietary_restriction::dsl::dietary_restriction
                    .filter(dietary_restriction::dsl::id.eq_any(diet_restriction_ids_list))
                    .select(DietaryRestriction::as_select())
                    .load(ts_conn)
                    .await?;

                Ok((ingredient, diet_restriction_list))
            },
        )
        .await?;

    Ok(result)
}

pub async fn search(app_state: &AppState, search: IngredientSearchForm) -> Result<Vec<Ingredient>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = ingredient::dsl::ingredient.select(Ingredient::as_select()).into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(ingredient::dsl::id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(ingredient::dsl::slug.eq(search_slug));
    }

    let result: Vec<Ingredient> = search_query.load(&mut conn).await?;

    Ok(result)
}
