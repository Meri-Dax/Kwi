use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::helpers::AppState;
use crate::{
    entities::ingredient::model::{Ingredient, IngredientForm},
    impl_insert,
};

impl_insert!(Ingredient, IngredientForm, crate::schema::ingredient::table);
