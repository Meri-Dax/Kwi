use diesel::{
    Selectable,
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    query_builder::AsChangeset,
};
use serde::{Deserialize, Serialize};

use crate::entities::ingredient::model::Ingredient;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::dietary_restriction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietaryRestriction {
    pub id: uuid::Uuid,
    pub slug: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::dietary_restriction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietaryRestrictionForm {
    pub slug: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::dietary_restriction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietaryRestrictionUpdateForm {
    pub slug: Option<String>,
}

#[derive(Deserialize, Queryable)]
#[diesel(table_name = dietary_restriction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietaryRestrictionSearchForm {
    pub id: Option<uuid::Uuid>,
    pub slug: Option<String>,
}

impl DietaryRestrictionSearchForm {
    pub fn empty() -> Self {
        Self { id: None, slug: None }
    }
    pub fn by_id(id: uuid::Uuid) -> Self {
        let mut res = Self::empty();

        res.id = Some(id);

        res
    }
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(table_name = crate::schema::ingredient_dietary_restriction)]
#[diesel(belongs_to(Ingredient))]
#[diesel(belongs_to(DietaryRestriction))]
#[diesel(primary_key(ingredient_id, dietary_restriction_id))]
pub struct IngredientDietaryRestriction {
    pub ingredient_id: uuid::Uuid,
    pub dietary_restriction_id: uuid::Uuid,
}

///
/// Web service structs
///
#[derive(serde::Deserialize)]
pub struct DietaryRestrictionWebForm {
    pub slug: String,
}

impl From<DietaryRestrictionWebForm> for DietaryRestrictionForm {
    fn from(DietaryRestrictionWebForm { slug }: DietaryRestrictionWebForm) -> Self {
        Self { slug }
    }
}

#[derive(serde::Deserialize)]
pub struct DietaryRestrictionUpdateWebForm {
    pub slug: Option<String>,
}

impl From<DietaryRestrictionUpdateWebForm> for DietaryRestrictionUpdateForm {
    fn from(DietaryRestrictionUpdateWebForm { slug }: DietaryRestrictionUpdateWebForm) -> Self {
        Self { slug }
    }
}

#[derive(Serialize)]
pub struct DietaryRestrictionWebView {
    pub id: uuid::Uuid,
    pub slug: String,
}

impl From<DietaryRestriction> for DietaryRestrictionWebView {
    fn from(DietaryRestriction { slug, id }: DietaryRestriction) -> Self {
        Self { slug, id }
    }
}
