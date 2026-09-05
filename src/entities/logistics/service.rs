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
