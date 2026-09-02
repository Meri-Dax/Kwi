use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::{self, model::DietaryRestriction},
        ingredient::{
            self,
            model::{Ingredient, IngredientForm, IngredientSearchForm, IngredientUpdateForm},
        },
    },
    helpers::AppState,
};

pub async fn insert(app_state: &AppState, form: IngredientForm) -> Result<Ingredient, RepositoryError> {
    ingredient::repository::insert(app_state, form).await
}

pub async fn list(app_state: &AppState, search_form: IngredientSearchForm) -> Result<Vec<Ingredient>, RepositoryError> {
    let ingredient = ingredient::repository::search(app_state, search_form).await?;

    Ok(ingredient)
}

pub async fn search_one(
    app_state: &AppState,
    search_form: IngredientSearchForm,
) -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
    let ingredient = ingredient::repository::search_one(app_state, search_form).await?;

    let diet_restrictions = dietary_restriction::repository::get_for_ingredient(app_state, &ingredient).await;

    match diet_restrictions {
        Ok(diet_restrictions) => Ok((ingredient, diet_restrictions)),
        Err(RepositoryError::NotFound) => Ok((ingredient, Vec::new())),
        Err(e) => Err(e),
    }
}

pub async fn update(
    app_state: &AppState,
    update_id: &uuid::Uuid,
    update_form: &IngredientUpdateForm,
    update_diets: &Option<Vec<uuid::Uuid>>,
) -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
    ingredient::repository::update_with_diet(app_state, update_id, update_form, update_diets).await
}

pub async fn insert_with_diet(
    app_state: &AppState,
    ingredient: IngredientForm,
    diet_restriction_list: Vec<uuid::Uuid>,
) -> Result<(Ingredient, Vec<DietaryRestriction>), RepositoryError> {
    ingredient::repository::insert_with_diet(app_state, ingredient, diet_restriction_list).await
}
