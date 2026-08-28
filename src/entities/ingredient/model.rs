use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
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
}

impl From<IngredientWebForm> for IngredientForm {
    fn from(IngredientWebForm { slug }: IngredientWebForm) -> Self {
        Self { slug }
    }
}

#[derive(serde::Serialize)]
pub struct IngredientWebView {
    pub id: uuid::Uuid,
    pub slug: String,
}

impl From<Ingredient> for IngredientWebView {
    fn from(Ingredient { slug, id }: Ingredient) -> Self {
        Self { slug, id }
    }
}
#[derive(Deserialize, Queryable)]
#[diesel(table_name = ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientSearchForm {
    pub id: Option<uuid::Uuid>,
    pub slug: Option<String>,
}
