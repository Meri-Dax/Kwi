use actix_web::{HttpResponse, Responder, post, web};
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}
