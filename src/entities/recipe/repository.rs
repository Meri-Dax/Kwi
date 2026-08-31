use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::DietaryRestriction,
        ingredient::model::{Ingredient, RecipeIngredient, RecipeIngredientForm, RecipeIngredientWebForm},
        recipe::model::{DetailedRecipe, Recipe, RecipeForm, RecipeSearchForm},
    },
    helpers::AppState,
    impl_insert,
    schema::{dietary_restriction, ingredient, ingredient_dietary_restriction, recipe, recipe_ingredient},
};

impl_insert!(Recipe, RecipeForm, crate::schema::recipe::table);

pub async fn search_one(app_state: &AppState, search: RecipeSearchForm) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut search_query = recipe::dsl::recipe.select(Recipe::as_select()).into_boxed();

    if let Some(search_id) = search.id {
        search_query = search_query.filter(recipe::dsl::id.eq(search_id));
    }

    if let Some(search_slug) = search.slug {
        search_query = search_query.filter(recipe::dsl::slug.eq(search_slug));
    }

    let mut tx = conn.build_transaction().read_only();

    let result = tx
        .run(async |ts_conn| -> Result<DetailedRecipe, RepositoryError> {
            let recipe: Recipe = search_query.first(ts_conn).await?;

            let ingredients: Vec<(RecipeIngredient, Ingredient)> = recipe_ingredient::dsl::recipe_ingredient
                .inner_join(ingredient::table)
                .filter(recipe_ingredient::dsl::recipe_id.eq(recipe.id))
                .select((RecipeIngredient::as_select(), Ingredient::as_select()))
                .load(ts_conn)
                .await?;

            let dietary_restrictions: Vec<DietaryRestriction> = dietary_restriction::table
                .inner_join(
                    ingredient_dietary_restriction::table
                        .on(ingredient_dietary_restriction::dietary_restriction_id.eq(dietary_restriction::id)),
                )
                .inner_join(ingredient::table.on(ingredient::id.eq(ingredient_dietary_restriction::ingredient_id)))
                .inner_join(recipe_ingredient::table.on(recipe_ingredient::ingredient_id.eq(ingredient::id)))
                .filter(recipe_ingredient::recipe_id.eq(&recipe.id))
                .select(DietaryRestriction::as_returning())
                .distinct()
                .load(ts_conn)
                .await?;

            Ok(DetailedRecipe {
                recipe,
                ingredients,
                dietary_restrictions,
            })
        })
        .await?;

    Ok(result)
}

pub async fn insert_with_ingredient(
    app_state: &AppState,
    recipe_form: RecipeForm,
    recipe_ingredient: Vec<RecipeIngredientWebForm>,
) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    let recipe = tx
        .run(async |ts_conn| -> Result<Recipe, RepositoryError> {
            let recipe = diesel::insert_into(recipe::table)
                .values(&recipe_form)
                .returning(Recipe::as_returning())
                .get_result(ts_conn)
                .await?;

            let recipe_ingredient_links: Vec<RecipeIngredientForm> = recipe_ingredient
                .iter()
                .map(|&recipe_ingredient| RecipeIngredientForm::from((&recipe, &recipe_ingredient)))
                .collect();

            diesel::insert_into(recipe_ingredient::table)
                .values(&recipe_ingredient_links)
                .execute(ts_conn)
                .await?;

            Ok(recipe)
        })
        .await?;

    let result = search_one(app_state, RecipeSearchForm::by_id(&recipe.id)).await?;

    Ok(result)
}
