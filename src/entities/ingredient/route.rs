use actix_web::{HttpResponse, Responder, get, patch, post, web};
use tracing::error;

use crate::{
    common::{http_response_message, repository::RepositoryError},
    entities::{
        dietary_restriction,
        ingredient::{
            self,
            model::{
                IngredientForm, IngredientUpdateForm, IngredientUpdateWebForm, IngredientWebForm, IngredientWebView,
            },
        },
    },
    helpers::AppState,
};

#[post("/ingredient")]
async fn create(app_state: web::Data<AppState>, payload_json: web::Json<IngredientWebForm>) -> impl Responder {
    let (ingredient, diet_restriction_list): (IngredientForm, Vec<uuid::Uuid>) = payload_json.into_inner().into();

    match ingredient::service::insert_with_diet(&app_state, ingredient, diet_restriction_list).await {
        Ok(ingredient_with_diet) => HttpResponse::Ok().json(IngredientWebView::from(ingredient_with_diet)),
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

#[patch("/ingredient/{id}")]
async fn update(
    app_state: web::Data<AppState>,
    update_id: web::Path<uuid::Uuid>,
    update_form: web::Json<IngredientUpdateWebForm>,
) -> impl Responder {
    let (update_form, update_diets): (IngredientUpdateForm, Option<Vec<uuid::Uuid>>) = update_form.into_inner().into();
    match ingredient::service::update(&app_state, &update_id, &update_form, &update_diets).await {
        Ok(result) => HttpResponse::Ok().json(IngredientWebView::from(result)),
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(e) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
    }
}

#[get("/ingredient/{id}")]
async fn view(app_state: web::Data<AppState>, search: web::Path<uuid::Uuid>) -> impl Responder {
    match ingredient::service::read(&app_state, &search.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(IngredientWebView::from(result)),
        Err(RepositoryError::NotFound) => http_response_message::NOT_FOUND.generic_response(),
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
        Err(e) => {
            error!("{}", e);
            http_response_message::INTERNAL_SERVER_ERROR.generic_response()
        }
    }
}

#[get("/ingredient")]
async fn list(app_state: web::Data<AppState>) -> impl Responder {
    let ingredients_list = match ingredient::service::list(&app_state).await {
        Ok(e) => e,
        Err(RepositoryError::NotFound) => {
            return HttpResponse::Ok().json(Vec::<IngredientWebView>::new());
        }
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            return http_response_message::INTERNAL_SERVER_ERROR.generic_response();
        }
        Err(_) => {
            return http_response_message::BAD_REQUEST.generic_response();
        }
    };

    let diet_restrictions = dietary_restriction::service::list_for_ingredients(&app_state, &ingredients_list).await;

    let diet_restrictions: Vec<IngredientWebView> = match diet_restrictions {
        Ok(list) => list.into_iter().map(Into::into).collect(),
        Err(RepositoryError::NotFound) => {
            return HttpResponse::Ok().json(Vec::<IngredientWebView>::new());
        }
        Err(RepositoryError::Database(e)) => {
            error!("{}", e);
            return http_response_message::INTERNAL_SERVER_ERROR.generic_response();
        }
        Err(_) => return http_response_message::BAD_REQUEST.generic_response(),
    };

    HttpResponse::Ok().json(diet_restrictions)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(view).service(list).service(update);
}
