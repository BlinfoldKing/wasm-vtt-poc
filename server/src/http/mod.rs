pub mod handler;
use actix_web::{web, HttpResponse, ResponseError};

use crate::app::App;
use crate::error::Error;

impl App {
    pub fn http_services(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/v1")
                .service(handler::ping::index)
                .service(handler::user::index),
        )
        .service(handler::index::index);
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Ok().json(&self)
    }
}
