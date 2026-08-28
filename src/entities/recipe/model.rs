use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
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

///
/// Web service structs
///
#[derive(serde::Deserialize)]
pub struct RecipeWebForm {
    pub slug: String,
}

impl From<RecipeWebForm> for RecipeForm {
    fn from(RecipeWebForm { slug }: RecipeWebForm) -> Self {
        Self { slug }
    }
}

#[derive(serde::Serialize)]
pub struct RecipeWebView {
    pub id: uuid::Uuid,
    pub slug: String,
}

impl From<Recipe> for RecipeWebView {
    fn from(Recipe { slug, id }: Recipe) -> Self {
        Self { slug, id }
    }
}
#[derive(Deserialize, Queryable)]
#[diesel(table_name = recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeSearchForm {
    pub id: Option<uuid::Uuid>,
    pub slug: Option<String>,
}
