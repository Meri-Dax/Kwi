use diesel::{Selectable, associations::Identifiable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::entities::dietary_restriction::model::{DietaryRestriction, DietaryRestrictionWebView};

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Clone)]
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
