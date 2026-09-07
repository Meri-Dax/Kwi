use std::collections::{HashMap, HashSet};

use chrono::Utc;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    common::{
        paginate::{List, Paginate},
        repository::RepositoryError,
    },
    entities::{
        course::model::{RecipeCourse, RecipeRecipeCourseForm, RecipeRecipeCourseWebForm},
        dietary_restriction::model::DietaryRestriction,
        ingredient::model::{Ingredient, RecipeIngredient, RecipeIngredientForm, RecipeIngredientWebForm},
        logistics::model::{RecipeLogistics, RecipeRecipeLogisticsForm, RecipeRecipeLogisticsWebForm},
        recipe::model::{DetailedRecipe, Recipe, RecipeForm, RecipeQuery, RecipeStatus, RecipeUpdateForm},
    },
    helpers::AppState,
    impl_insert,
    schema::{
        dietary_restriction, ingredient, ingredient_dietary_restriction, recipe, recipe_course, recipe_ingredient,
        recipe_logistics, recipe_recipe_course_xref, recipe_recipe_logistics_xref,
    },
};

impl_insert!(Recipe, RecipeForm, crate::schema::recipe::table);

pub async fn read(app_state: &AppState, search_id: &uuid::Uuid) -> Result<DetailedRecipe, RepositoryError> {
    let search_array = vec![*search_id];
    let result = get_from_list(app_state, &search_array).await?;

    match result.is_empty() {
        true => Err(RepositoryError::NotFound),
        false => Ok(result[0].clone()),
    }
}

pub async fn insert_with_xref(
    app_state: &AppState,
    recipe_form: &RecipeForm,
    recipe_ingredients: &Vec<RecipeIngredientWebForm>,
    recipe_logistics: &Vec<RecipeRecipeLogisticsWebForm>,
    recipe_courses: &Vec<RecipeRecipeCourseWebForm>,
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

                diesel::insert_into(recipe_recipe_logistics_xref::table)
                    .values(&recipe_logistics_links)
                    .execute(ts_conn)
                    .await?;
            }

            if !recipe_courses.is_empty() {
                let recipe_courses_links: Vec<RecipeRecipeCourseForm> = recipe_courses
                    .iter()
                    .map(|&rl| RecipeRecipeCourseForm::from((&recipe, &rl)))
                    .collect();

                diesel::insert_into(recipe_recipe_course_xref::table)
                    .values(&recipe_courses_links)
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
    recipe_logistics: &Option<Vec<RecipeRecipeLogisticsWebForm>>,
    recipe_courses: &Option<Vec<RecipeRecipeCourseWebForm>>,
) -> Result<DetailedRecipe, RepositoryError> {
    let mut conn = app_state.database.get().await?;
    let mut tx = conn.build_transaction().read_write();

    tx.run(async |ts_conn| -> Result<(), RepositoryError> {
        if !recipe_form.is_empty() {
            diesel::update(recipe::table.filter(recipe::id.eq(recipe_id)))
                .set((recipe_form, recipe::date_updated.eq(Utc::now().naive_utc())))
                .execute(ts_conn)
                .await?;
        }

        if let Some(recipe_ingredients) = recipe_ingredients {
            diesel::delete(recipe_ingredient::table.filter(recipe_ingredient::recipe_id.eq(recipe_id)))
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

        if let Some(recipe_logistics) = recipe_logistics {
            diesel::delete(
                recipe_recipe_logistics_xref::table.filter(recipe_recipe_logistics_xref::recipe_id.eq(recipe_id)),
            )
            .execute(ts_conn)
            .await?;

            if !recipe_logistics.is_empty() {
                let recipe_logistics_links: Vec<RecipeRecipeLogisticsForm> = recipe_logistics
                    .iter()
                    .map(|&recipe_logistics| RecipeRecipeLogisticsForm::from((recipe_id, &recipe_logistics)))
                    .collect();

                diesel::insert_into(recipe_recipe_logistics_xref::table)
                    .values(&recipe_logistics_links)
                    .execute(ts_conn)
                    .await?;
            }
        }

        if let Some(recipe_courses) = recipe_courses {
            diesel::delete(recipe_recipe_course_xref::table.filter(recipe_recipe_course_xref::recipe_id.eq(recipe_id)))
                .execute(ts_conn)
                .await?;

            if !recipe_courses.is_empty() {
                let recipe_course_links: Vec<RecipeRecipeCourseForm> = recipe_courses
                    .iter()
                    .map(|&recipe_course| RecipeRecipeCourseForm::from((recipe_id, &recipe_course)))
                    .collect();

                diesel::insert_into(recipe_recipe_course_xref::table)
                    .values(&recipe_course_links)
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
        search: _, // TODO: implement txt search
        exclude_dietary_restriction,
        exclude_logistics,
    } = query;

    let page = page.unwrap_or_else(|| RecipeQuery::default().page.unwrap());

    let mut query = recipe::table
        .select(recipe::id)
        .order(recipe::date_created.desc())
        .filter(recipe::status.eq(RecipeStatus::Public))
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
            .filter(ingredient_dietary_restriction::dietary_restriction_id.eq_any(excluded_diet_ids))
            .select(recipe_ingredient::recipe_id)
            .into_boxed();

        query = query.filter(recipe::id.ne_all(excluded_recipe_ids));
    }

    if let Some(exclude_logistics) = exclude_logistics
        && !exclude_logistics.is_empty()
    {
        let excluded_recipe_ids = recipe::table
            .inner_join(recipe_recipe_logistics_xref::table.on(recipe_recipe_logistics_xref::recipe_id.eq(recipe::id)))
            .filter(recipe_recipe_logistics_xref::recipe_logistics_id.eq_any(exclude_logistics))
            .select(recipe::id)
            .into_boxed();

        query = query.filter(recipe::id.ne_all(excluded_recipe_ids));
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
        Option<RecipeCourse>,
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
        .left_join(recipe_recipe_course_xref::table.on(recipe_recipe_course_xref::recipe_id.eq(recipe::id)))
        .left_join(recipe_course::table.on(recipe_course::id.eq(recipe_recipe_course_xref::recipe_course_id)))
        .filter(recipe::id.eq_any(ids_list))
        .select((
            Recipe::as_returning(),
            Option::<DietaryRestriction>::as_returning(),
            Option::<RecipeIngredient>::as_returning(),
            Option::<Ingredient>::as_returning(),
            Option::<RecipeLogistics>::as_returning(),
            Option::<RecipeCourse>::as_returning(),
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
            HashSet<RecipeCourse>,
        ),
    > = HashMap::new();

    for (recipe, diet, recipe_ingredient, ingredient, logistics, course) in result {
        let entry = grouped
            .entry(recipe.id)
            .or_insert_with(|| (recipe, HashMap::new(), HashSet::new(), HashSet::new(), HashSet::new()));

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
        if let Some(course) = course {
            entry.4.insert(course);
        }
    }

    let ordered: Vec<DetailedRecipe> = ids_list
        .iter()
        .map(|&id| {
            let (recipe, ingredients_map, dietary_restrictions_set, recipe_logistics_set, recipe_courses_set) =
                grouped.remove(&id).expect(&format!("Broken request {:?}", id));

            DetailedRecipe {
                recipe,
                ingredients: ingredients_map.into_values().collect(),
                dietary_restrictions: dietary_restrictions_set.into_iter().collect(),
                logistics: recipe_logistics_set.into_iter().collect(),
                courses: recipe_courses_set.into_iter().collect(),
            }
        })
        .collect();

    Ok(ordered)
}
