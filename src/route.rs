use actix_web::web;

use crate::entities::ingredient;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(ingredient::route::config));
}
