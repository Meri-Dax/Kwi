use actix_web::web;

use crate::entities::{dietary_restriction, ingredient, logistics, recipe};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/cook")
                .configure(ingredient::route::config)
                .configure(dietary_restriction::route::config)
                .configure(logistics::route::config)
                .configure(recipe::route::config),
        ),
    );
}
