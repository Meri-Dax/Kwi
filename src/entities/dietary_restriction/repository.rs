use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper, associations::GroupedBy};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::{
            DietaryRestriction, DietaryRestrictionForm, DietaryRestrictionUpdateForm, IngredientDietaryRestriction,
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

pub async fn update(
    app_state: &AppState,
    id: &uuid::Uuid,
    form: &DietaryRestrictionUpdateForm,
) -> Result<DietaryRestriction, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result =
        diesel::update(dietary_restriction::dsl::dietary_restriction.filter(dietary_restriction::dsl::id.eq(id)))
            .set(form)
            .returning(DietaryRestriction::as_returning())
            .get_result(&mut conn)
            .await?;

    Ok(result)
}

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<DietaryRestriction, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = dietary_restriction::dsl::dietary_restriction
        .select(DietaryRestriction::as_select())
        .filter(dietary_restriction::dsl::id.eq(search_id))
        .first(&mut conn)
        .await?;

    Ok(result)
}

pub async fn list(app_state: &AppState) -> Result<Vec<DietaryRestriction>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = dietary_restriction::dsl::dietary_restriction
        .select(DietaryRestriction::as_select())
        .load(&mut conn)
        .await?;

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
