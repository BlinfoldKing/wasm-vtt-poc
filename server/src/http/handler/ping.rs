use crate::{error::Error, http::Response};
use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Ping<'a> {
    ping: &'a str,
}

#[get("/ping")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(Response::ok(Ping { ping: "pong" }))
}
