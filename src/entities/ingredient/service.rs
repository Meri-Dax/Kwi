use crate::{
    common::repository::Error as RepositoryError,
    entities::ingredient::{
        self,
        model::{Ingredient, IngredientForm},
    },
    helpers::AppState,
};

pub async fn insert(
    app_state: &AppState,
    form: IngredientForm,
) -> Result<Ingredient, RepositoryError> {
    let result = ingredient::repository::insert(app_state, form).await?;

    Ok(result)
}
