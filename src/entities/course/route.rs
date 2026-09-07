use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::course::{
        self,
        model::{RecipeCourseWebForm, RecipeCourseWebView},
    },
    helpers::AppState,
};

#[post("/course")]
pub async fn create(app_state: web::Data<AppState>, payload: web::Json<RecipeCourseWebForm>) -> impl Responder {
    match course::service::create(&app_state, &payload.into_inner().into()).await {
        Ok(e) => HttpResponse::Ok().json(RecipeCourseWebView::from(e)),
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

#[get("/course/{id}")]
pub async fn get(app_state: web::Data<AppState>, search_id: web::Path<uuid::Uuid>) -> impl Responder {
    match course::service::read(&app_state, &search_id.into_inner()).await {
        Ok(e) => HttpResponse::Ok().json(RecipeCourseWebView::from(e)),
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

#[get("/course")]
pub async fn list(app_state: web::Data<AppState>) -> impl Responder {
    match course::service::list(&app_state).await {
        Ok(list) => {
            let list: Vec<RecipeCourseWebView> = list.into_iter().map(RecipeCourseWebView::from).collect();
            HttpResponse::Ok().json(list)
        }
        Err(RepositoryError::NotFound) => HttpResponse::Ok().json(Vec::<RecipeCourseWebView>::new()),
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
