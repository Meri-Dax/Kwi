// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ingredient_unit"))]
    pub struct IngredientUnit;
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
    recipe (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
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

diesel::joinable!(ingredient_dietary_restriction -> dietary_restriction (dietary_restriction_id));
diesel::joinable!(ingredient_dietary_restriction -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    dietary_restriction,
    ingredient,
    ingredient_dietary_restriction,
    recipe,
    recipe_ingredient,
);
