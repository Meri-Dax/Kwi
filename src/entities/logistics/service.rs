use crate::{
    common::repository::RepositoryError,
    entities::logistics::{
        self,
        model::{RecipeLogistics, RecipeLogisticsForm},
    },
    helpers::AppState,
};

pub async fn create(app_state: &AppState, form: &RecipeLogisticsForm) -> Result<RecipeLogistics, RepositoryError> {
    logistics::repository::insert(app_state, form).await
}

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<RecipeLogistics, RepositoryError> {
    logistics::repository::read(app_state, search_id).await
}

pub async fn list(app_state: &AppState) -> Result<Vec<RecipeLogistics>, RepositoryError> {
    logistics::repository::list(app_state).await
}
