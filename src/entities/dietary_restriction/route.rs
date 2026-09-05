use actix_web::{HttpResponse, Responder, get, patch, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::dietary_restriction::{
        self,
        model::{
            DietaryRestrictionUpdateForm, DietaryRestrictionUpdateWebForm, DietaryRestrictionWebForm,
            DietaryRestrictionWebView,
        },
    },
    helpers::AppState,
};

#[get("/diet/{id}")]
async fn view(app_state: web::Data<AppState>, search: web::Path<uuid::Uuid>) -> impl Responder {
    match dietary_restriction::service::read(&app_state, &search.into_inner()).await {
        Ok(diet) => HttpResponse::Ok().json(DietaryRestrictionWebView::from(diet)),
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

#[patch("/diet/{id}")]
async fn update(
    app_state: web::Data<AppState>,
    update_id: web::Path<uuid::Uuid>,
    payload_json: web::Json<DietaryRestrictionUpdateWebForm>,
) -> impl Responder {
    let payload: DietaryRestrictionUpdateForm = payload_json.into_inner().into();

    match dietary_restriction::service::update(&app_state, &update_id, &payload).await {
        Ok(dietary_restriction) => HttpResponse::Ok().json(DietaryRestrictionWebView::from(dietary_restriction)),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
        Err(e) => {
            error!("{}", e);
            http_response_message::BAD_REQUEST.generic_response()
        }
    }
}

#[post("/diet")]
async fn create(app_state: web::Data<AppState>, payload: web::Json<DietaryRestrictionWebForm>) -> impl Responder {
    match dietary_restriction::service::insert(&app_state, &payload.into_inner().into()).await {
        Ok(dietary_restriction) => HttpResponse::Ok().json(DietaryRestrictionWebView::from(dietary_restriction)),
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

#[get("/diet")]
async fn list(app_state: web::Data<AppState>) -> impl Responder {
    match dietary_restriction::service::list(&app_state).await {
        Ok(list) => {
            let public_view: Vec<DietaryRestrictionWebView> = list.into_iter().map(Into::into).collect();

            HttpResponse::Ok().json(public_view)
        }
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
        Err(e) => {
            error!("{}", e);
            http_response_message::BAD_REQUEST.generic_response()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(list).service(view).service(update);
}
