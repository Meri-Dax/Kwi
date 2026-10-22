use actix_web::{HttpResponse, Responder, get, patch, post, web};
use serde_qs::web::QsQuery;
use tracing::error;

use crate::{
    common::{http_response_message, paginate::List, repository::RepositoryError},
    entities::{
        course::model::RecipeRecipeCourseWebForm,
        ingredient::model::RecipeIngredientWebForm,
        logistics::model::RecipeRecipeLogisticsWebForm,
        recipe::{
            self,
            model::{
                RecipeForm, RecipeQuery, RecipeUpdateForm, RecipeUpdateWebForm, RecipeViewQuery, RecipeWebForm,
                RecipeWebView,
            },
        },
    },
    helpers::AppState,
};

#[post("/recipe")]
async fn create(app_state: web::Data<AppState>, payload_json: web::Json<RecipeWebForm>) -> impl Responder {
    let (recipe, ingredients, logistics, courses, servings): (
        RecipeForm,
        Vec<RecipeIngredientWebForm>,
        Vec<RecipeRecipeLogisticsWebForm>,
        Vec<RecipeRecipeCourseWebForm>,
        i16,
    ) = payload_json.into_inner().into();

    match recipe::service::insert_with_xref(&app_state, &recipe, &ingredients, &logistics, &courses).await {
        Ok(recipe) => HttpResponse::Ok().json(RecipeWebView::from((recipe, servings))),
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
    let (recipe, ingredients, logistics, courses): (
        RecipeUpdateForm,
        Option<Vec<RecipeIngredientWebForm>>,
        Option<Vec<RecipeRecipeLogisticsWebForm>>,
        Option<Vec<RecipeRecipeCourseWebForm>>,
    ) = payload_json.into_inner().into();

    match recipe::service::update_with_xref(&app_state, &id, &recipe, &ingredients, &logistics, &courses).await {
        Ok(recipe) => HttpResponse::Ok().json(RecipeWebView::from((recipe, 1))),
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
async fn view(
    app_state: web::Data<AppState>,
    search: web::Path<uuid::Uuid>,
    query: QsQuery<RecipeViewQuery>,
) -> impl Responder {
    let RecipeViewQuery { servings } = query.into_inner();

    match recipe::service::search_one(&app_state, &search.into_inner()).await {
        Ok(recipe) => HttpResponse::Ok().json(RecipeWebView::from((recipe, servings.unwrap_or(1)))),
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
async fn list(app_state: web::Data<AppState>, search: QsQuery<RecipeQuery>) -> impl Responder {
    let search = search.into_inner();
    match recipe::service::list(&app_state, &search).await {
        Ok(recipe_list) => HttpResponse::Ok().json(List::<RecipeWebView>::from((
            recipe_list,
            search.servings.unwrap_or(1).into(),
        ))),
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
