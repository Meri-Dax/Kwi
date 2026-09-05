use std::collections::{HashMap, HashSet};

use chrono::Utc;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use tracing::info;

use crate::{
    common::{
        paginate::{List, Paginate},
        repository::RepositoryError,
    },
    entities::{
        dietary_restriction::model::DietaryRestriction,
        ingredient::model::{Ingredient, RecipeIngredient, RecipeIngredientForm, RecipeIngredientWebForm},
        logistics::model::{RecipeLogistics, RecipeRecipeLogisticsForm, RecipeRecipeLogisticsWebForm},
        recipe::model::{DetailedRecipe, Recipe, RecipeForm, RecipeQuery, RecipeUpdateForm},
    },
    helpers::AppState,
    impl_insert,
    schema::{
        dietary_restriction, ingredient, ingredient_dietary_restriction, recipe, recipe_ingredient, recipe_logistics,
        recipe_recipe_logistics_xref,
    },
};

impl_insert!(Recipe, RecipeForm, crate::schema::recipe::table);

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let mut tx = conn.build_transaction().read_only();

    let result = tx
        .run(async |ts_conn| -> Result<DetailedRecipe, RepositoryError> {
            let recipe: Recipe = recipe::dsl::recipe
                .select(Recipe::as_select())
                .filter(recipe::dsl::id.eq(search_id))
                .first(ts_conn)
                .await?;

            let ingredients: Vec<(RecipeIngredient, Ingredient)> = recipe_ingredient::dsl::recipe_ingredient
                .inner_join(ingredient::table)
                .filter(recipe_ingredient::dsl::recipe_id.eq(search_id))
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
                .filter(recipe_ingredient::recipe_id.eq(search_id))
                .select(DietaryRestriction::as_returning())
                .distinct()
                .load(ts_conn)
                .await?;

            let logistics: Vec<RecipeLogistics> = recipe_logistics::table
                .inner_join(
                    recipe_recipe_logistics_xref::table
                        .on(recipe_recipe_logistics_xref::recipe_logistics_id.eq(recipe_logistics::id)),
                )
                .filter(recipe_recipe_logistics_xref::recipe_id.eq(search_id))
                .select(RecipeLogistics::as_returning())
                .distinct()
                .load(ts_conn)
                .await?;

            Ok(DetailedRecipe {
                recipe,
                ingredients,
                dietary_restrictions,
                logistics,
            })
        })
        .await?;

    Ok(result)
}

pub async fn insert_with_xref(
    app_state: &AppState,
    recipe_form: &RecipeForm,
    recipe_ingredients: &Vec<RecipeIngredientWebForm>,
    recipe_logistics: &Vec<RecipeRecipeLogisticsWebForm>,
) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    let recipe = tx
        .run(async |ts_conn| -> Result<Recipe, RepositoryError> {
            let recipe = diesel::insert_into(recipe::table)
                .values(recipe_form)
                .returning(Recipe::as_returning())
                .get_result(ts_conn)
                .await?;

            if !recipe_ingredients.is_empty() {
                let recipe_ingredient_links: Vec<RecipeIngredientForm> = recipe_ingredients
                    .iter()
                    .map(|&recipe_ingredient| RecipeIngredientForm::from((&recipe, &recipe_ingredient)))
                    .collect();

                diesel::insert_into(recipe_ingredient::table)
                    .values(&recipe_ingredient_links)
                    .execute(ts_conn)
                    .await?;
            }

            if !recipe_logistics.is_empty() {
                let recipe_logistics_links: Vec<RecipeRecipeLogisticsForm> = recipe_logistics
                    .iter()
                    .map(|&rl| RecipeRecipeLogisticsForm::from((&recipe, &rl)))
                    .collect();
                info!("Attempting to insert: {:?}", recipe_logistics_links);

                diesel::insert_into(recipe_recipe_logistics_xref::table)
                    .values(&recipe_logistics_links)
                    .execute(ts_conn)
                    .await?;
            }

            Ok(recipe)
        })
        .await?;

    let result = read(app_state, &recipe.id).await?;

    Ok(result)
}

pub async fn update_with_xref(
    app_state: &AppState,
    recipe_id: &uuid::Uuid,
    recipe_form: &RecipeUpdateForm,
    recipe_ingredients: &Option<Vec<RecipeIngredientWebForm>>,
) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    tx.run(async |ts_conn| -> Result<(), RepositoryError> {
        if !recipe_form.is_empty() {
            diesel::update(recipe::table.filter(recipe::dsl::id.eq(recipe_id)))
                .set((recipe_form, recipe::dsl::date_updated.eq(Utc::now().naive_utc())))
                .execute(ts_conn)
                .await?;
        }

        if let Some(recipe_ingredients) = recipe_ingredients {
            diesel::delete(recipe_ingredient::table.filter(recipe_ingredient::dsl::recipe_id.eq(recipe_id)))
                .execute(ts_conn)
                .await?;

            if !recipe_ingredients.is_empty() {
                let recipe_ingredient_links: Vec<RecipeIngredientForm> = recipe_ingredients
                    .iter()
                    .map(|&recipe_ingredient| RecipeIngredientForm::from((recipe_id, &recipe_ingredient)))
                    .collect();

                diesel::insert_into(recipe_ingredient::table)
                    .values(&recipe_ingredient_links)
                    .execute(ts_conn)
                    .await?;
            }
        }

        Ok(())
    })
    .await?;

    let result = read(app_state, recipe_id).await?;

    Ok(result)
}

pub async fn list(app_state: &AppState, query: &RecipeQuery) -> Result<List<uuid::Uuid>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let RecipeQuery {
        page,
        search: _,
        exclude_dietary_restriction,
    } = query;

    let page = page.unwrap_or_else(|| RecipeQuery::default().page.unwrap());

    let mut query = recipe::dsl::recipe
        .select(recipe::dsl::id)
        .order(recipe::date_created.desc())
        .into_boxed();

    if let Some(excluded_diet_ids) = exclude_dietary_restriction
        && !excluded_diet_ids.is_empty()
    {
        let excluded_recipe_ids = recipe_ingredient::table
            .inner_join(ingredient::table.on(ingredient::id.eq(recipe_ingredient::ingredient_id)))
            .inner_join(
                ingredient_dietary_restriction::table
                    .on(ingredient_dietary_restriction::ingredient_id.eq(ingredient::id)),
            )
            .filter(ingredient_dietary_restriction::dsl::dietary_restriction_id.eq_any(excluded_diet_ids))
            .select(recipe_ingredient::dsl::recipe_id)
            .into_boxed();

        query = query.filter(recipe::dsl::id.ne_all(excluded_recipe_ids));
    }

    let result = query
        .paginate(page)
        .per_page(10)
        .load_and_count_pages::<uuid::Uuid>(&mut conn)
        .await?;

    Ok(result)
}

pub async fn get_from_list(
    app_state: &AppState,
    ids_list: &Vec<uuid::Uuid>,
) -> Result<Vec<DetailedRecipe>, RepositoryError> {
    let mut conn = app_state.database.get().await?;

    let result: Vec<(
        Recipe,
        Option<DietaryRestriction>,
        Option<RecipeIngredient>,
        Option<Ingredient>,
        Option<RecipeLogistics>,
    )> = recipe::table
        .left_join(recipe_recipe_logistics_xref::table.on(recipe_recipe_logistics_xref::recipe_id.eq(recipe::id)))
        .left_join(
            recipe_logistics::table.on(recipe_logistics::id.eq(recipe_recipe_logistics_xref::recipe_logistics_id)),
        )
        .left_join(recipe_ingredient::table.on(recipe_ingredient::recipe_id.eq(recipe::id)))
        .left_join(ingredient::table.on(ingredient::id.eq(recipe_ingredient::ingredient_id)))
        .left_join(
            ingredient_dietary_restriction::table.on(ingredient_dietary_restriction::ingredient_id.eq(ingredient::id)),
        )
        .left_join(
            dietary_restriction::table
                .on(dietary_restriction::id.eq(ingredient_dietary_restriction::dietary_restriction_id)),
        )
        .filter(recipe::dsl::id.eq_any(ids_list))
        .select((
            Recipe::as_returning(),
            Option::<DietaryRestriction>::as_returning(),
            Option::<RecipeIngredient>::as_returning(),
            Option::<Ingredient>::as_returning(),
            Option::<RecipeLogistics>::as_returning(),
        ))
        .load(&mut conn)
        .await?;

    let mut grouped: HashMap<
        uuid::Uuid,
        (
            Recipe,
            HashMap<uuid::Uuid, (RecipeIngredient, Ingredient)>,
            HashSet<DietaryRestriction>,
            HashSet<RecipeLogistics>,
        ),
    > = HashMap::new();

    for (recipe, diet, recipe_ingredient, ingredient, logistics) in result {
        let entry = grouped
            .entry(recipe.id)
            .or_insert_with(|| (recipe, HashMap::new(), HashSet::new(), HashSet::new()));

        if let Some(ingredient) = ingredient
            && let Some(recipe_ingredient) = recipe_ingredient
        {
            entry.1.insert(ingredient.id, (recipe_ingredient, ingredient));
        }
        if let Some(diet) = diet {
            entry.2.insert(diet);
        }
        if let Some(logistics) = logistics {
            entry.3.insert(logistics);
        }
    }
    info!("ids {:?}", ids_list);
    info!("recipes {:?}", grouped);

    let ordered: Vec<DetailedRecipe> = ids_list
        .iter()
        .map(|&id| {
            let (recipe, ingredients_map, dietary_restrictions_set, recipe_logistics_set) =
                grouped.remove(&id).expect(&format!("Broken request {:?}", id));

            DetailedRecipe {
                recipe,
                ingredients: ingredients_map.into_values().collect(),
                dietary_restrictions: dietary_restrictions_set.into_iter().collect(),
                logistics: recipe_logistics_set.into_iter().collect(),
            }
        })
        .collect();

    Ok(ordered)
}
