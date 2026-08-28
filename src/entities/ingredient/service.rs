use crate::{
    common::repository::RepositoryError,
    entities::ingredient::{
        self,
        model::{Ingredient, IngredientForm, IngredientSearchForm},
    },
    helpers::AppState,
};

pub async fn insert(
    app_state: &AppState,
    form: IngredientForm,
) -> Result<Ingredient, RepositoryError> {
    ingredient::repository::insert(app_state, form).await
}

pub async fn search_one(
    app_state: &AppState,
    search_form: IngredientSearchForm,
) -> Result<Ingredient, RepositoryError> {
    ingredient::repository::search_one(&app_state, search_form).await
}
