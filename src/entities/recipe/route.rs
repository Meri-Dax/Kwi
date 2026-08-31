use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::{
        ingredient::model::RecipeIngredientWebForm,
        recipe::{
            self,
            model::{RecipeForm, RecipeSearchForm, RecipeWebForm, RecipeWebView},
        },
    },
    helpers::AppState,
};

#[post("/recipe")]
async fn create(app_state: web::Data<AppState>, payload_json: web::Json<RecipeWebForm>) -> impl Responder {
    let (recipe, ingredients): (RecipeForm, Vec<RecipeIngredientWebForm>) = payload_json.into_inner().into();

    match recipe::service::insert_with_ingredients(&app_state, recipe, ingredients).await {
        Ok(recipe) => HttpResponse::Ok().json(RecipeWebView::from(recipe)),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(e) => {
            error!("{}", e);
            http_response_message::BAD_REQUEST.generic_response()
        }
    }
}

#[get("/recipe/{id}")]
async fn view(app_state: web::Data<AppState>, search: web::Path<uuid::Uuid>) -> impl Responder {
    match recipe::service::search_one(
        &app_state,
        RecipeSearchForm {
            id: Some(search.into_inner()),
            slug: None,
        },
    )
    .await
    {
        Ok(recipe) => HttpResponse::Ok().json(RecipeWebView::from(recipe)),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
        Err(e) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(view);
}
