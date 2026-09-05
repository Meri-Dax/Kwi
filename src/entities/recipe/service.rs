use crate::{
    common::{paginate::List, repository::RepositoryError},
    entities::{
        ingredient::model::RecipeIngredientWebForm,
        logistics::model::RecipeRecipeLogisticsWebForm,
        recipe::{
            self,
            model::{DetailedRecipe, Recipe, RecipeForm, RecipeQuery, RecipeUpdateForm},
        },
    },
    helpers::AppState,
};

pub async fn insert(app_state: &AppState, form: &RecipeForm) -> Result<Recipe, RepositoryError> {
    recipe::repository::insert(app_state, form).await
}

pub async fn insert_with_ingredients(
    app_state: &AppState,
    recipe_form: &RecipeForm,
    ingredients: &Vec<RecipeIngredientWebForm>,
    logistics: &Vec<RecipeRecipeLogisticsWebForm>,
) -> Result<DetailedRecipe, RepositoryError> {
    recipe::repository::insert_with_xref(app_state, recipe_form, ingredients, logistics).await
}

pub async fn update_with_ingredients(
    app_state: &AppState,
    id: &uuid::Uuid,
    recipe_form: &RecipeUpdateForm,
    ingredients: &Option<Vec<RecipeIngredientWebForm>>,
) -> Result<DetailedRecipe, RepositoryError> {
    recipe::repository::update_with_xref(app_state, id, recipe_form, ingredients).await
}

pub async fn search_one(app_state: &AppState, search_id: &uuid::Uuid) -> Result<DetailedRecipe, RepositoryError> {
    recipe::repository::read(app_state, search_id).await
}

pub async fn list(app_state: &AppState, query: &RecipeQuery) -> Result<List<DetailedRecipe>, RepositoryError> {
    let List {
        list: ids_list,
        max_page,
        page,
    } = recipe::repository::list(app_state, query).await?;

    let list = recipe::repository::get_from_list(app_state, &ids_list).await?;

    Ok(List { list, max_page, page })
}
