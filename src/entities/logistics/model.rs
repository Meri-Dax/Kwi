use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::entities::recipe::model::Recipe;
// use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[diesel(table_name = crate::schema::recipe_logistics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeLogistics {
    pub id: uuid::Uuid,
    pub slug: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_logistics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeLogisticsForm {
    pub slug: String,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_recipe_logistics_xref)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeRecipeLogistics {
    pub recipe_id: uuid::Uuid,
    pub recipe_logistics_id: uuid::Uuid,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_recipe_logistics_xref)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeRecipeLogisticsForm {
    pub recipe_id: uuid::Uuid,
    pub recipe_logistics_id: uuid::Uuid,
}

impl From<(&Recipe, &RecipeRecipeLogisticsWebForm)> for RecipeRecipeLogisticsForm {
    fn from((recipe, recipe_logistics): (&Recipe, &RecipeRecipeLogisticsWebForm)) -> Self {
        Self {
            recipe_id: recipe.id,
            recipe_logistics_id: recipe_logistics.id,
        }
    }
}

///
/// Web service structs
///
#[derive(Deserialize)]
pub struct RecipeLogisticsWebForm {
    pub slug: String,
}

impl From<RecipeLogisticsWebForm> for RecipeLogisticsForm {
    fn from(RecipeLogisticsWebForm { slug }: RecipeLogisticsWebForm) -> Self {
        Self { slug }
    }
}

#[derive(Serialize)]
pub struct RecipeLogisticsWebView {
    pub id: uuid::Uuid,
    pub slug: String,
}

impl From<RecipeLogistics> for RecipeLogisticsWebView {
    fn from(req: RecipeLogistics) -> Self {
        Self {
            id: req.id,
            slug: req.slug,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecipeRecipeLogisticsWebForm {
    pub id: uuid::Uuid,
}

impl From<uuid::Uuid> for RecipeRecipeLogisticsWebForm {
    fn from(id: uuid::Uuid) -> Self {
        Self { id }
    }
}
