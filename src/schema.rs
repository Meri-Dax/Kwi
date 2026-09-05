// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ingredient_unit"))]
    pub struct IngredientUnit;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "recipe_status"))]
    pub struct RecipeStatus;
}

diesel::table! {
    dietary_restriction (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
        fresh_for_days -> Nullable<Int2>,
    }
}

diesel::table! {
    ingredient_dietary_restriction (id) {
        id -> Uuid,
        ingredient_id -> Uuid,
        dietary_restriction_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RecipeStatus;

    recipe (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
        status -> RecipeStatus,
        prep_time -> Nullable<Int2>,
        cook_time -> Nullable<Int2>,
        fresh_for_hours -> Nullable<Int2>,
        steps -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    recipe_course (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::table! {
    recipe_equipment (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IngredientUnit;

    recipe_ingredient (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        ingredient_id -> Uuid,
        qty -> Int4,
        unit -> IngredientUnit,
    }
}

diesel::table! {
    recipe_logistics (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::table! {
    recipe_recipe_course_xref (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        recipe_course_id -> Uuid,
    }
}

diesel::table! {
    recipe_recipe_equipment_xref (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        recipe_equipment_id -> Uuid,
    }
}

diesel::table! {
    recipe_recipe_logistics_xref (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        recipe_logistics_id -> Uuid,
    }
}

diesel::joinable!(ingredient_dietary_restriction -> dietary_restriction (dietary_restriction_id));
diesel::joinable!(ingredient_dietary_restriction -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(recipe_recipe_course_xref -> recipe (recipe_id));
diesel::joinable!(recipe_recipe_course_xref -> recipe_course (recipe_course_id));
diesel::joinable!(recipe_recipe_equipment_xref -> recipe (recipe_id));
diesel::joinable!(recipe_recipe_equipment_xref -> recipe_equipment (recipe_equipment_id));
diesel::joinable!(recipe_recipe_logistics_xref -> recipe (recipe_id));
diesel::joinable!(recipe_recipe_logistics_xref -> recipe_logistics (recipe_logistics_id));

diesel::allow_tables_to_appear_in_same_query!(
    dietary_restriction,
    ingredient,
    ingredient_dietary_restriction,
    recipe,
    recipe_course,
    recipe_equipment,
    recipe_ingredient,
    recipe_logistics,
    recipe_recipe_course_xref,
    recipe_recipe_equipment_xref,
    recipe_recipe_logistics_xref,
);
