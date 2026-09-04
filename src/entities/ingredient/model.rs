use diesel::{
    Associations, Selectable, associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::entities::{
    dietary_restriction::model::{DietaryRestriction, DietaryRestrictionWebView},
    recipe::model::Recipe,
};

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingredient {
    pub id: uuid::Uuid,
    pub slug: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientForm {
    pub slug: String,
}

#[derive(AsChangeset, Default, PartialEq)]
#[diesel(table_name = crate::schema::ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientUpdateForm {
    pub slug: Option<String>,
}

impl IngredientUpdateForm {
    pub fn is_empty(&self) -> bool {
        *self == IngredientUpdateForm::default()
    }
}

#[derive(DbEnum, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[ExistingTypePath = "crate::schema::sql_types::IngredientUnit"]
#[serde(rename_all = "kebab-case")]
pub enum IngredientUnit {
    Unit,
    #[serde(alias = "ml")]
    Milliliter,
    #[serde(alias = "g")]
    Gram,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(table_name = crate::schema::recipe_ingredient)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Ingredient))]
#[diesel(primary_key(recipe_id, ingredient_id))]
pub struct RecipeIngredient {
    pub recipe_id: uuid::Uuid,
    pub ingredient_id: uuid::Uuid,
    pub qty: i32,
    pub unit: IngredientUnit,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::recipe_ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeIngredientForm {
    pub recipe_id: uuid::Uuid,
    pub ingredient_id: uuid::Uuid,
    pub qty: i32,
    pub unit: IngredientUnit,
}

///
/// Web service structs
///
#[derive(serde::Deserialize)]
pub struct IngredientWebForm {
    pub slug: String,
    pub dietary_restrictions: Option<Vec<uuid::Uuid>>,
}

impl From<IngredientWebForm> for (IngredientForm, Vec<uuid::Uuid>) {
    fn from(
        IngredientWebForm {
            slug,
            dietary_restrictions,
        }: IngredientWebForm,
    ) -> (IngredientForm, Vec<uuid::Uuid>) {
        (
            IngredientForm { slug },
            match dietary_restrictions {
                Some(diet) => diet,
                None => Vec::new(),
            },
        )
    }
}

#[derive(serde::Deserialize)]
pub struct IngredientUpdateWebForm {
    pub slug: Option<String>,
    pub dietary_restrictions: Option<Vec<uuid::Uuid>>,
}

impl From<IngredientUpdateWebForm> for (IngredientUpdateForm, Option<Vec<uuid::Uuid>>) {
    fn from(
        IngredientUpdateWebForm {
            slug,
            dietary_restrictions,
        }: IngredientUpdateWebForm,
    ) -> (IngredientUpdateForm, Option<Vec<uuid::Uuid>>) {
        (IngredientUpdateForm { slug }, dietary_restrictions)
    }
}

#[derive(Serialize)]
pub struct IngredientWebView {
    pub id: uuid::Uuid,
    pub slug: String,
    pub dietary_restrictions: Vec<DietaryRestrictionWebView>,
}

impl From<(Ingredient, Vec<DietaryRestriction>)> for IngredientWebView {
    fn from((Ingredient { slug, id }, diet): (Ingredient, Vec<DietaryRestriction>)) -> Self {
        Self {
            slug,
            id,
            dietary_restrictions: diet.into_iter().map(DietaryRestrictionWebView::from).collect(),
        }
    }
}

#[derive(Deserialize, Queryable)]
#[diesel(table_name = ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientSearchForm {
    pub id: Option<uuid::Uuid>,
    pub slug: Option<String>,
}

impl IngredientSearchForm {
    pub fn empty() -> Self {
        Self { id: None, slug: None }
    }
}

#[derive(Serialize)]
pub struct RecipeIngredientWebView {
    pub id: uuid::Uuid,
    pub slug: String,
    pub qty: i32,
    pub unit: IngredientUnit,
}

impl From<(RecipeIngredient, Ingredient)> for RecipeIngredientWebView {
    fn from(
        (
            RecipeIngredient {
                qty,
                unit,
                recipe_id: _,
                ingredient_id: _,
            },
            Ingredient { id, slug },
        ): (RecipeIngredient, Ingredient),
    ) -> Self {
        Self { id, slug, qty, unit }
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct RecipeIngredientWebForm {
    pub id: uuid::Uuid,
    pub qty: i32,
    pub unit: IngredientUnit,
}

impl From<(&Recipe, &RecipeIngredientWebForm)> for RecipeIngredientForm {
    fn from(
        (
            &Recipe {
                id: recipe_id,
                slug: _,
                date_created: _,
                date_updated: _,
            },
            &RecipeIngredientWebForm {
                id: ingredient_id,
                qty,
                unit,
            },
        ): (&Recipe, &RecipeIngredientWebForm),
    ) -> Self {
        Self {
            recipe_id,
            ingredient_id,
            qty,
            unit,
        }
    }
}

impl From<(&uuid::Uuid, &RecipeIngredientWebForm)> for RecipeIngredientForm {
    fn from(
        (
            recipe_id,
            &RecipeIngredientWebForm {
                id: ingredient_id,
                qty,
                unit,
            },
        ): (&uuid::Uuid, &RecipeIngredientWebForm),
    ) -> Self {
        Self {
            recipe_id: *recipe_id,
            ingredient_id,
            qty,
            unit,
        }
    }
}
