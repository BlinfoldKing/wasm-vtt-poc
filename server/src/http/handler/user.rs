use std::sync::{Arc, Mutex};

use crate::usecases::Usecase;
use crate::{error::Error, http::Response};
use actix_web::{get, web, HttpResponse, Result};

#[get("/users")]
async fn index(usecase: web::Data<Arc<Mutex<Usecase>>>) -> Result<HttpResponse, Error> {
    let users = usecase.lock().unwrap().user.get_user_list()?;
    Ok(Response::ok(users))
}
