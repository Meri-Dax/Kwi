use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::logistics::{
        self,
        model::{RecipeLogisticsWebForm, RecipeLogisticsWebView},
    },
    helpers::AppState,
};

#[post("/logistics")]
pub async fn create(app_state: web::Data<AppState>, payload: web::Json<RecipeLogisticsWebForm>) -> impl Responder {
    match logistics::service::create(&app_state, &payload.into_inner().into()).await {
        Ok(e) => HttpResponse::Ok().json(RecipeLogisticsWebView::from(e)),
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

#[get("/logistics/{id}")]
pub async fn get(app_state: web::Data<AppState>, search_id: web::Path<uuid::Uuid>) -> impl Responder {
    match logistics::service::read(&app_state, &search_id.into_inner()).await {
        Ok(e) => HttpResponse::Ok().json(RecipeLogisticsWebView::from(e)),
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
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

#[get("/logistics")]
pub async fn list(app_state: web::Data<AppState>) -> impl Responder {
    match logistics::service::list(&app_state).await {
        Ok(list) => {
            let list: Vec<RecipeLogisticsWebView> = list.into_iter().map(RecipeLogisticsWebView::from).collect();
            HttpResponse::Ok().json(list)
        }
        Err(RepositoryError::NotFound) => HttpResponse::Ok().json(Vec::<RecipeLogisticsWebView>::new()),
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(get).service(list);
}
