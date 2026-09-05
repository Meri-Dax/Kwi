use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::{
            self,
            model::{DietaryRestriction, DietaryRestrictionForm, DietaryRestrictionUpdateForm},
        },
        ingredient::model::Ingredient,
    },
    helpers::AppState,
};

pub async fn insert(
    app_state: &AppState,
    form: &DietaryRestrictionForm,
) -> Result<DietaryRestriction, RepositoryError> {
    dietary_restriction::repository::insert(app_state, form).await
}

pub async fn update(
    app_state: &AppState,
    id: &uuid::Uuid,
    form: &DietaryRestrictionUpdateForm,
) -> Result<DietaryRestriction, RepositoryError> {
    dietary_restriction::repository::update(app_state, id, form).await
}

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<DietaryRestriction, RepositoryError> {
    dietary_restriction::repository::read(app_state, search_id).await
}

pub async fn list(app_state: &AppState) -> Result<Vec<DietaryRestriction>, RepositoryError> {
    dietary_restriction::repository::list(app_state).await
}

pub async fn list_for_ingredients(
    app_state: &AppState,
    ingredients: &Vec<Ingredient>,
) -> Result<Vec<(Ingredient, Vec<DietaryRestriction>)>, RepositoryError> {
    let result = dietary_restriction::repository::list_for_ingredients(app_state, ingredients).await?;

    Ok(result)
}
