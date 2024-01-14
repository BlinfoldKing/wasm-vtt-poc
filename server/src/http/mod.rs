pub mod handler;
use actix_web::{web, App as HttpApp, HttpResponse, HttpServer, ResponseError, Result};

use crate::app::App;
use crate::error::{Error, ErrorKind};
use crate::repository::sqlite::SqliteRepository;
use crate::usecases::setting::SettingUsecase;
use crate::usecases::user::UserUsecase;

impl App {
    pub async fn serve(&self) -> Result<(), Error> {
        let user = UserUsecase::<SqliteRepository>::new(self.config.clone())?;
        let setting = SettingUsecase::<SqliteRepository>::new(self.config.clone())?;

        let server = HttpServer::new(move || {
            let app = HttpApp::new()
                .service(
                    web::scope("/v1")
                        .app_data(web::Data::new(user.clone()))
                        .app_data(web::Data::new(setting.clone()))
                        .service(handler::ping::index)
                        .service(handler::user::index),
                )
                .service(handler::index::index)
                .service(handler::index::dist);
            app
        })
        .bind(("0.0.0.0", self.config.http.port))
        .map_err(|err| Error::new(err.to_string(), ErrorKind::Http))?
        .run();

        println!("server running on {}", self.config.http.port);
        server
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Http))
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
