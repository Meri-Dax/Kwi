use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

use crate::{
    common::paginate::{List, deserialize_opt_page},
    entities::{
        dietary_restriction::model::DietaryRestriction,
        ingredient::model::{Ingredient, RecipeIngredient, RecipeIngredientWebForm, RecipeIngredientWebView},
    },
    helpers::empty_string_as_none,
};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub slug: String,
    pub steps: Option<String>,
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeForm {
    pub slug: String,
    pub steps: Option<String>,
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
}

#[derive(AsChangeset, Default, PartialEq)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeUpdateForm {
    pub slug: Option<String>,
    pub steps: Option<String>,
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
}

impl RecipeUpdateForm {
    pub fn is_empty(&self) -> bool {
        *self == RecipeUpdateForm::default()
    }
}

pub struct DetailedRecipe {
    pub recipe: Recipe,
    pub ingredients: Vec<(RecipeIngredient, Ingredient)>,
    pub dietary_restrictions: Vec<DietaryRestriction>,
}

///
/// Web service structs
///
#[derive(serde::Deserialize)]
pub struct RecipeWebForm {
    pub slug: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub steps: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
    pub ingredients: Vec<RecipeIngredientWebForm>,
}

impl From<RecipeWebForm> for (RecipeForm, Vec<RecipeIngredientWebForm>) {
    fn from(
        RecipeWebForm {
            slug,
            steps,
            description,
            prep_time,
            cook_time,
            fresh_for_hours,
            ingredients,
        }: RecipeWebForm,
    ) -> Self {
        (
            RecipeForm {
                slug,
                steps,
                description,
                prep_time,
                cook_time,
                fresh_for_hours,
            },
            ingredients,
        )
    }
}

#[derive(serde::Deserialize)]
pub struct RecipeUpdateWebForm {
    pub slug: Option<String>,
    pub steps: Option<String>,
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
    pub ingredients: Option<Vec<RecipeIngredientWebForm>>,
}

impl From<RecipeUpdateWebForm> for (RecipeUpdateForm, Option<Vec<RecipeIngredientWebForm>>) {
    fn from(
        RecipeUpdateWebForm {
            slug,
            steps,
            description,
            prep_time,
            cook_time,
            fresh_for_hours,
            ingredients,
        }: RecipeUpdateWebForm,
    ) -> Self {
        (
            RecipeUpdateForm {
                slug,
                steps,
                description,
                prep_time,
                cook_time,
                fresh_for_hours,
            },
            ingredients,
        )
    }
}

#[derive(Serialize)]
pub struct RecipeWebView {
    pub id: uuid::Uuid,
    pub slug: String,
    pub steps: Option<String>,
    pub description: Option<String>,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub fresh_for_hours: Option<i16>,
    pub ingredients: Vec<RecipeIngredientWebView>,
    pub dietary_restrictions: Vec<DietaryRestriction>,
}

impl From<DetailedRecipe> for RecipeWebView {
    fn from(
        DetailedRecipe {
            recipe,
            ingredients,
            dietary_restrictions,
        }: DetailedRecipe,
    ) -> Self {
        Self {
            slug: recipe.slug,
            id: recipe.id,
            steps: recipe.steps,
            description: recipe.description,
            prep_time: recipe.prep_time,
            cook_time: recipe.cook_time,
            fresh_for_hours: recipe.fresh_for_hours,
            ingredients: ingredients.into_iter().map(RecipeIngredientWebView::from).collect(),
            dietary_restrictions,
        }
    }
}

impl From<List<DetailedRecipe>> for List<RecipeWebView> {
    fn from(List { page, max_page, list }: List<DetailedRecipe>) -> Self {
        List {
            page,
            max_page,
            list: list.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct RecipeQuery {
    #[serde(default, deserialize_with = "deserialize_opt_page")]
    pub page: Option<i64>,
    pub search: Option<String>,
    pub exclude_dietary_restriction: Option<Vec<uuid::Uuid>>,
}

impl Default for RecipeQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            search: None,
            exclude_dietary_restriction: None,
        }
    }
}
