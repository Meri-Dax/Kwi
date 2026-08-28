// @generated automatically by Diesel CLI.

diesel::table! {
    ingredient (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
    }
}

diesel::table! {
    recipe (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ingredient,
    recipe,
);
