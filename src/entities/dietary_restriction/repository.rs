use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper, associations::GroupedBy};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::{
            DietaryRestriction, DietaryRestrictionForm, DietaryRestrictionSearchForm, IngredientDietaryRestriction,
        },
        ingredient::model::Ingredient,
    },
    helpers::AppState,
    impl_insert,
    schema::dietary_restriction,
};

impl_insert!(
    DietaryRestriction,
    DietaryRestrictionForm,
    crate::schema::dietary_restriction::table
);

pub async fn insert_multiple(
    app_state: &AppState,
    diet_list: &Vec<DietaryRestrictionForm>,
) -> Result<(), RepositoryError> {
    let mut conn = app_state.database.get().await?;

    diesel::insert_into(dietary_restriction::table)
        .values(diet_list)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn search_one(
    app_state: &AppState,
    search: DietaryRestrictionSearchForm,
) -> Result<DietaryRestriction, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = dietary_restriction::dsl::dietary_restriction
        .select(DietaryRestriction::as_select())
        .into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(dietary_restriction::dsl::id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(dietary_restriction::dsl::slug.eq(search_slug));
    }

    let result: DietaryRestriction = search_query.first(&mut conn).await?;

    Ok(result)
}

pub async fn search(
    app_state: &AppState,
    search: DietaryRestrictionSearchForm,
) -> Result<Vec<DietaryRestriction>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = dietary_restriction::dsl::dietary_restriction
        .select(DietaryRestriction::as_select())
        .into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(dietary_restriction::dsl::id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(dietary_restriction::dsl::slug.eq(search_slug));
    }

    let result: Vec<DietaryRestriction> = search_query.load(&mut conn).await?;

    Ok(result)
}

pub async fn get_for_ingredient(
    app_state: &AppState,
    ingredient: &Ingredient,
) -> Result<Vec<DietaryRestriction>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = IngredientDietaryRestriction::belonging_to(ingredient)
        .inner_join(dietary_restriction::table)
        .select(DietaryRestriction::as_select())
        .load(&mut conn)
        .await?;

    Ok(result)
}

pub async fn list_for_ingredients(
    app_state: &AppState,
    ingredients: &Vec<Ingredient>,
) -> Result<Vec<(Ingredient, Vec<DietaryRestriction>)>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let ingredients_with_diets: Vec<(IngredientDietaryRestriction, DietaryRestriction)> =
        IngredientDietaryRestriction::belonging_to(ingredients)
            .inner_join(dietary_restriction::table)
            .select((
                IngredientDietaryRestriction::as_select(),
                DietaryRestriction::as_select(),
            ))
            .load(&mut conn)
            .await?;

    let grouped: Vec<Vec<DietaryRestriction>> = ingredients_with_diets
        .grouped_by(ingredients)
        .into_iter()
        .map(|pairs| pairs.into_iter().map(|(_join, diet)| diet).collect())
        .collect();

    let result: Vec<(Ingredient, Vec<DietaryRestriction>)> = ingredients.iter().cloned().zip(grouped).collect();

    Ok(result)
}
