use actix_web::{HttpResponse, Responder, get, patch, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::{
        ingredient::model::RecipeIngredientWebForm,
        recipe::{
            self,
            model::{
                RecipeForm, RecipeSearchForm, RecipeUpdateForm, RecipeUpdateWebForm, RecipeWebForm, RecipeWebView,
            },
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

#[patch("/recipe/{id}")]
async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<uuid::Uuid>,
    payload_json: web::Json<RecipeUpdateWebForm>,
) -> impl Responder {
    let (recipe, ingredients): (RecipeUpdateForm, Option<Vec<RecipeIngredientWebForm>>) =
        payload_json.into_inner().into();

    match recipe::service::update_with_ingredients(&app_state, &id, &recipe, &ingredients).await {
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
    cfg.service(create).service(view).service(update);
}
