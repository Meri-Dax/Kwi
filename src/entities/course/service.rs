use crate::{
    common::repository::RepositoryError,
    entities::course::{
        self,
        model::{RecipeCourse, RecipeCourseForm},
    },
    helpers::AppState,
};

pub async fn create(app_state: &AppState, form: &RecipeCourseForm) -> Result<RecipeCourse, RepositoryError> {
    course::repository::insert(app_state, form).await
}

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<RecipeCourse, RepositoryError> {
    course::repository::read(app_state, search_id).await
}

pub async fn list(app_state: &AppState) -> Result<Vec<RecipeCourse>, RepositoryError> {
    course::repository::list(app_state).await
}
