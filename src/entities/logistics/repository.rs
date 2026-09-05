use crate::{
    entities::logistics::model::{RecipeLogistics, RecipeLogisticsForm},
    helpers::AppState,
    impl_insert,
};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

impl_insert!(
    RecipeLogistics,
    RecipeLogisticsForm,
    crate::schema::recipe_logistics::table
);
