use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::entities::recipe::model::Recipe;

#[derive(Queryable, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[diesel(table_name = crate::schema::recipe_course)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeCourse {
    pub id: uuid::Uuid,
    pub slug: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_course)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeCourseForm {
    pub slug: String,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_recipe_course_xref)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeRecipeCourse {
    pub recipe_id: uuid::Uuid,
    pub recipe_course_id: uuid::Uuid,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::recipe_recipe_course_xref)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeRecipeCourseForm {
    pub recipe_id: uuid::Uuid,
    pub recipe_course_id: uuid::Uuid,
}

impl From<(&Recipe, &RecipeRecipeCourseWebForm)> for RecipeRecipeCourseForm {
    fn from((recipe, recipe_course): (&Recipe, &RecipeRecipeCourseWebForm)) -> Self {
        Self {
            recipe_id: recipe.id,
            recipe_course_id: recipe_course.id,
        }
    }
}

impl From<(&uuid::Uuid, &RecipeRecipeCourseWebForm)> for RecipeRecipeCourseForm {
    fn from((recipe_id, recipe_course): (&uuid::Uuid, &RecipeRecipeCourseWebForm)) -> Self {
        Self {
            recipe_id: *recipe_id,
            recipe_course_id: recipe_course.id,
        }
    }
}

///
/// Web service structs
///
#[derive(Deserialize)]
pub struct RecipeCourseWebForm {
    pub slug: String,
}

impl From<RecipeCourseWebForm> for RecipeCourseForm {
    fn from(RecipeCourseWebForm { slug }: RecipeCourseWebForm) -> Self {
        Self { slug }
    }
}

#[derive(Serialize)]
pub struct RecipeCourseWebView {
    pub id: uuid::Uuid,
    pub slug: String,
}

impl From<RecipeCourse> for RecipeCourseWebView {
    fn from(req: RecipeCourse) -> Self {
        Self {
            id: req.id,
            slug: req.slug,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct RecipeRecipeCourseWebForm {
    pub id: uuid::Uuid,
}

impl From<uuid::Uuid> for RecipeRecipeCourseWebForm {
    fn from(id: uuid::Uuid) -> Self {
        Self { id }
    }
}
