use std::sync::{Arc, Mutex};

pub use actix_web::web::Json as JsonRequest;
use actix_web::HttpResponse;

pub use super::Response;
pub use crate::error::{Error, ErrorKind};
use crate::usecases::Usecase;
pub use actix_web::web::ReqData;
pub use actix_web::{delete, get, patch, post, put, web};

pub type EndpointResult = Result<HttpResponse, Error>;
pub type SharedUsecase = web::Data<Arc<Mutex<Usecase>>>;
