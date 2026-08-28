use crate::{
    common::repository::RepositoryError,
    entities::recipe::{
        self,
        model::{Recipe, RecipeForm, RecipeSearchForm},
    },
    helpers::AppState,
};

pub async fn insert(
    app_state: &AppState,
    form: RecipeForm,
) -> Result<Recipe, RepositoryError> {
    recipe::repository::insert(app_state, form).await
}

pub async fn search_one(
    app_state: &AppState,
    search_form: RecipeSearchForm,
) -> Result<Recipe, RepositoryError> {
    recipe::repository::search_one(&app_state, search_form).await
}
