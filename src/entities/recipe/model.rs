use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

use crate::entities::{
    dietary_restriction::model::DietaryRestriction,
    ingredient::model::{Ingredient, RecipeIngredient, RecipeIngredientWebForm, RecipeIngredientWebView},
};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub slug: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeForm {
    pub slug: String,
}

#[derive(AsChangeset, Default, PartialEq)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeUpdateForm {
    pub slug: Option<String>,
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
    pub ingredients: Vec<RecipeIngredientWebForm>,
}

impl From<RecipeWebForm> for (RecipeForm, Vec<RecipeIngredientWebForm>) {
    fn from(RecipeWebForm { slug, ingredients }: RecipeWebForm) -> Self {
        (RecipeForm { slug }, ingredients)
    }
}

#[derive(serde::Deserialize)]
pub struct RecipeUpdateWebForm {
    pub slug: Option<String>,
    pub ingredients: Option<Vec<RecipeIngredientWebForm>>,
}

impl From<RecipeUpdateWebForm> for (RecipeUpdateForm, Option<Vec<RecipeIngredientWebForm>>) {
    fn from(RecipeUpdateWebForm { slug, ingredients }: RecipeUpdateWebForm) -> Self {
        (RecipeUpdateForm { slug }, ingredients)
    }
}

#[derive(Serialize)]
pub struct RecipeWebView {
    pub id: uuid::Uuid,
    pub slug: String,
    pub ingredients: Vec<RecipeIngredientWebView>,
    pub dietary_restrictions: Vec<DietaryRestriction>,
}

impl From<DetailedRecipe> for RecipeWebView {
    fn from(
        DetailedRecipe {
            recipe: Recipe { slug, id },
            ingredients,
            dietary_restrictions,
        }: DetailedRecipe,
    ) -> Self {
        Self {
            slug,
            id,
            ingredients: ingredients.into_iter().map(RecipeIngredientWebView::from).collect(),
            dietary_restrictions,
        }
    }
}

#[derive(Deserialize, Queryable)]
#[diesel(table_name = recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeSearchForm {
    pub id: Option<uuid::Uuid>,
    pub slug: Option<String>,
}

impl RecipeSearchForm {
    pub fn by_id(recipe_id: &uuid::Uuid) -> Self {
        Self {
            slug: None,
            id: Some(*recipe_id),
        }
    }
}
