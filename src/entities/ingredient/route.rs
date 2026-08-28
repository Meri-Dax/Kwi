use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::ingredient::{
        self,
        model::{
            IngredientForm, IngredientSearchForm, IngredientWebForm,
            IngredientWebView,
        },
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
        Ok(ingredient) => {
            HttpResponse::Ok().json(IngredientWebView::from(ingredient))
        }
        Err(e) => {
            error!("{}", e);
            http_response_message::BAD_REQUEST.generic_response()
        }
    }
}

#[get("/ingredient/{id}")]
async fn view(
    app_state: web::Data<AppState>,
    search: web::Path<uuid::Uuid>,
) -> impl Responder {
    match ingredient::service::search_one(
        &app_state,
        IngredientSearchForm {
            id: Some(search.into_inner()),
            slug: None,
        },
    )
    .await
    {
        Ok(ingredient) => {
            HttpResponse::Ok().json(IngredientWebView::from(ingredient))
        }
        Err(RepositoryError::NotFound) => {
            http_response_message::NOT_FOUND.generic_response()
        }
        Err(e) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(view);
}
