pub mod handler;
pub mod middlewares;
pub mod prelude;

use actix_web::{web, HttpResponse, ResponseError};
use serde::Serialize;

use crate::app::App;
use crate::error::{Error, ErrorKind};

impl App {
    pub fn http_services(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/v1")
                .wrap(middlewares::auth::Auth)
                .service(web::scope("/auth").service(handler::auth::index))
                .service(
                    web::scope("/users")
                        .service(handler::user::index)
                        .service(handler::user::me),
                )
                .service(web::scope("/ping").service(handler::ping::index)),
        )
        .service(handler::index::index);
    }
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub message: String,
    pub data: Option<T>,
    pub ok: bool,
}

impl<T: Serialize> Response<T> {
    pub fn ok_json(self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Ok().json(self)
    }

    pub fn ok(data: T) -> HttpResponse<actix_web::body::BoxBody> {
        Self {
            message: "success".to_owned(),
            data: Some(data),
            ok: true,
        }
        .ok_json()
    }
}

impl Response<Error> {
    pub fn err(error: Error) -> HttpResponse<actix_web::body::BoxBody> {
        let message = error.message.clone();
        Response::<Error> {
            message,
            data: None,
            ok: false,
        }
        .err_json(error.kind)
    }

    pub fn err_json(self, kind: ErrorKind) -> HttpResponse<actix_web::body::BoxBody> {
        let mut http_resp = match kind {
            ErrorKind::NotFound => HttpResponse::NotFound(),
            ErrorKind::Unauthenticated | ErrorKind::Unauthorized => HttpResponse::Unauthorized(),
            ErrorKind::BadRequest => HttpResponse::BadRequest(),
            _ => HttpResponse::InternalServerError(),
        };

        http_resp.json(self)
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
