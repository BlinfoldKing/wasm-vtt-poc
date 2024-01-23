pub mod handler;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub message: String,
    pub data: Option<T>,
    pub error: bool,
}

impl<T: Serialize> Response<T> {
    fn json(self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Ok().json(self)
    }

    pub fn ok(data: T) -> HttpResponse<actix_web::body::BoxBody> {
        Self {
            message: "success".to_owned(),
            data: Some(data),
            error: false,
        }
        .json()
    }
}

impl Response<Error> {
    pub fn err(error: Error) -> HttpResponse<actix_web::body::BoxBody> {
        Response::<Error> {
            message: "success".to_owned(),
            data: Some(error),
            error: false,
        }
        .json()
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        Response::err(self.clone())
    }
}
