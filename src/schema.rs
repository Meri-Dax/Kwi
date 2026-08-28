// @generated automatically by Diesel CLI.

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
        id -> Int4,
        ingredient_id -> Uuid,
        diet_id -> Uuid,
    }
}

diesel::table! {
    recipe (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
    }
}

diesel::joinable!(ingredient_dietary_restriction -> dietary_restriction (diet_id));
diesel::joinable!(ingredient_dietary_restriction -> ingredient (ingredient_id));

diesel::allow_tables_to_appear_in_same_query!(
    dietary_restriction,
    ingredient,
    ingredient_dietary_restriction,
    recipe,
);
