use crate::error::Error;
use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Ping<'a> {
    ping: &'a str,
}

#[get("/ping")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Ping { ping: "pong" }))
}
