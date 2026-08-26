use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpResponseMessage<'a> {
    pub message: &'a str,
    pub code: usize,
}

impl<'a> From<&'a str> for HttpResponseMessage<'a> {
    fn from(req: &'a str) -> Self {
        HttpResponseMessage {
            message: req,
            code: 400,
        }
    }
}

macro_rules! http_response_message {
    ($message:expr, $code: expr) => {
        HttpResponseMessage {
            message: $message,
            code: $code,
        }
    };
}

pub const OK: HttpResponseMessage = http_response_message!("Ok", 200);
pub const BAD_REQUEST: HttpResponseMessage =
    http_response_message!("Bad Request", 400);
pub const UNAUTHORIZED: HttpResponseMessage =
    http_response_message!("Unauthorized", 401);
pub const FORBIDDEN: HttpResponseMessage =
    http_response_message!("Forbidden", 403);
pub const NOT_FOUND: HttpResponseMessage =
    http_response_message!("Not Found", 404);
pub const INTERNAL_SERVER_ERROR: HttpResponseMessage =
    http_response_message!("Internal Server Error", 500);

impl HttpResponseMessage<'_> {
    pub fn generic_response(&self) -> HttpResponse {
        let status = StatusCode::from_u16(self.code as u16)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status).json(self)
    }
    pub fn response(&self, message: &str) -> HttpResponse {
        let status = StatusCode::from_u16(self.code as u16)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        HttpResponse::build(status).json(HttpResponseMessage {
            code: self.code,
            message,
        })
    }
}
