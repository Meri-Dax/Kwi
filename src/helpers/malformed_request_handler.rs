use actix_web::web;

use crate::common::http_response_message::BAD_REQUEST;

pub fn json() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| {
        let response = BAD_REQUEST.response(&err.to_string());
        actix_web::error::InternalError::from_response(err, response).into()
    })
}

pub fn path() -> web::PathConfig {
    web::PathConfig::default().error_handler(|err, _req| {
        let response = BAD_REQUEST.response(&err.to_string());
        actix_web::error::InternalError::from_response(err, response).into()
    })
}
