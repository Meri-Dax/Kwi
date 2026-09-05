use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Clone, Debug)]
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
