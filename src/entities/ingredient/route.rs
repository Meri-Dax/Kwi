use actix_web::{Responder, get, post, web};
use tracing::error;

use crate::{
    common::http_response_message,
    entities::ingredient::{
        self,
        model::{IngredientForm, IngredientWebForm},
    },
    helpers::AppState,
};

#[post("/ingredient")]
async fn create(
    app_state: web::Data<AppState>,
    payload_json: web::Json<IngredientWebForm>,
) -> impl Responder {
    let payload: IngredientForm = payload_json.into_inner().into();

    match ingredient::service::insert(&app_state, payload).await {
        Ok(_) => http_response_message::OK.generic_response(),
        Err(e) => {
            error!("{}", e);
            http_response_message::BAD_REQUEST.generic_response()
        }
    }
}

#[get("/ingredient/{id}")]
async fn view(
    _app_state: web::Data<AppState>,
    _id: web::Path<uuid::Uuid>,
) -> impl Responder {
    http_response_message::OK.generic_response()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(view);
}
