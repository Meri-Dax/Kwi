// @generated automatically by Diesel CLI.

diesel::table! {
    ingredient (id) {
        id -> Uuid,
        #[max_length = 255]
        slug -> Varchar,
    }
}
