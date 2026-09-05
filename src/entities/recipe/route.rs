use actix_web::{HttpRequest, HttpResponse, Responder, get, patch, post, web};
use serde_qs::web::QsQuery;
use tracing::{error, info};

use crate::{
    common::{http_response_message, paginate::List, repository::RepositoryError},
    entities::{
        ingredient::model::RecipeIngredientWebForm,
        logistics::model::RecipeRecipeLogisticsWebForm,
        recipe::{
            self,
            model::{RecipeForm, RecipeQuery, RecipeUpdateForm, RecipeUpdateWebForm, RecipeWebForm, RecipeWebView},
        },
    },
    helpers::AppState,
};

#[post("/recipe")]
async fn create(app_state: web::Data<AppState>, payload_json: web::Json<RecipeWebForm>) -> impl Responder {
    let (recipe, ingredients, logistics): (
        RecipeForm,
        Vec<RecipeIngredientWebForm>,
        Vec<RecipeRecipeLogisticsWebForm>,
    ) = payload_json.into_inner().into();

    match recipe::service::insert_with_ingredients(&app_state, &recipe, &ingredients, &logistics).await {
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
    match recipe::service::search_one(&app_state, &search.into_inner()).await {
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

#[get("/recipe")]
async fn list(req: HttpRequest, app_state: web::Data<AppState>, search: QsQuery<RecipeQuery>) -> impl Responder {
    let search = search.into_inner();
    info!("raw query string: {:?}", req.query_string());

    info!("query: {:?}", &search);
    match recipe::service::list(&app_state, &search).await {
        Ok(recipe_list) => HttpResponse::Ok().json(List::<RecipeWebView>::from(recipe_list)),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(RepositoryError::NotFound) => HttpResponse::Ok().json(List::<RecipeWebView>::empty()),
        Err(e) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(view).service(update).service(list);
}
