use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::{
            self,
            model::{DietaryRestriction, DietaryRestrictionForm, DietaryRestrictionSearchForm},
        },
        ingredient::model::Ingredient,
    },
    helpers::AppState,
};

pub async fn insert(app_state: &AppState, form: DietaryRestrictionForm) -> Result<DietaryRestriction, RepositoryError> {
    dietary_restriction::repository::insert(app_state, form).await
}

pub async fn search_one(
    app_state: &AppState,
    search_form: DietaryRestrictionSearchForm,
) -> Result<DietaryRestriction, RepositoryError> {
    dietary_restriction::repository::search_one(&app_state, search_form).await
}

pub async fn list(app_state: &AppState) -> Result<Vec<DietaryRestriction>, RepositoryError> {
    dietary_restriction::repository::search(app_state, DietaryRestrictionSearchForm::empty()).await
}

pub async fn list_for_ingredients(
    app_state: &AppState,
    ingredients: &Vec<Ingredient>,
) -> Result<Vec<(Ingredient, Vec<DietaryRestriction>)>, RepositoryError> {
    let result = dietary_restriction::repository::list_for_ingredients(app_state, ingredients).await?;

    Ok(result)
}
