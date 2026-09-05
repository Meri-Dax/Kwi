use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::{DietaryRestriction, IngredientDietaryRestriction},
        ingredient::model::{Ingredient, IngredientForm, IngredientUpdateForm},
    },
    helpers::AppState,
    impl_insert,
    schema::{dietary_restriction, ingredient, ingredient_dietary_restriction},
};

impl_insert!(Ingredient, IngredientForm, crate::schema::ingredient::table);

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<Ingredient, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = ingredient::dsl::ingredient
        .select(Ingredient::as_select())
        .filter(ingredient::dsl::id.eq(search_id))
        .first(&mut conn)
        .await?;

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

pub async fn update_with_diet(
    app_state: &AppState,
    id: &uuid::Uuid,
    form: &IngredientUpdateForm,
    diets_list: &Option<Vec<uuid::Uuid>>,
) -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    let result = tx
        .run(
            async |ts_conn| -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
                // Update the ingredient
                let ingredient = match form.is_empty() {
                    true => {
                        ingredient::dsl::ingredient
                            .select(Ingredient::as_select())
                            .filter(ingredient::dsl::id.eq(id))
                            .first(ts_conn)
                            .await?
                    }
                    false => {
                        diesel::update(ingredient::dsl::ingredient.filter(ingredient::dsl::id.eq(id)))
                            .set(form)
                            .returning(Ingredient::as_returning())
                            .get_result(ts_conn)
                            .await?
                    }
                };

                let dietary_restriction_list = match diets_list {
                    Some(v) if v.is_empty() => {
                        // Provided diet list is empty; delete any relationship
                        diesel::delete(
                            ingredient_dietary_restriction::dsl::ingredient_dietary_restriction
                                .filter(ingredient_dietary_restriction::dsl::ingredient_id.eq(id)),
                        )
                        .execute(ts_conn)
                        .await?;

                        Vec::new()
                    }
                    Some(v) => {
                        // Update the provided relations with the new list

                        // Delete the ingredient <-> diet links
                        diesel::delete(
                            ingredient_dietary_restriction::dsl::ingredient_dietary_restriction
                                .filter(ingredient_dietary_restriction::dsl::ingredient_id.eq(id)),
                        )
                        .execute(ts_conn)
                        .await?;

                        // Re-create the ingredient <-> diet links
                        let diet_restriction_links: Vec<IngredientDietaryRestriction> = v
                            .iter()
                            .map(|&dietary_restriction_id| IngredientDietaryRestriction {
                                dietary_restriction_id,
                                ingredient_id: ingredient.id.clone(),
                            })
                            .collect();

                        diesel::insert_into(ingredient_dietary_restriction::table)
                            .values(&diet_restriction_links)
                            .execute(ts_conn)
                            .await?;

                        // Select the linked diets
                        dietary_restriction::dsl::dietary_restriction
                            .filter(dietary_restriction::dsl::id.eq_any(v))
                            .select(DietaryRestriction::as_select())
                            .load(ts_conn)
                            .await?
                    }
                    None => {
                        // No update requested, list the updated fields
                        dietary_restriction::dsl::dietary_restriction
                            .inner_join(ingredient_dietary_restriction::table)
                            .filter(ingredient_dietary_restriction::dsl::ingredient_id.eq(id))
                            .select(DietaryRestriction::as_select())
                            .load(ts_conn)
                            .await?
                    }
                };

                Ok((ingredient, dietary_restriction_list))
            },
        )
        .await?;

    Ok(result)
}

pub async fn list(app_state: &AppState) -> Result<Vec<Ingredient>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result = ingredient::dsl::ingredient
        .select(Ingredient::as_select())
        .load(&mut conn)
        .await?;

    Ok(result)
}
