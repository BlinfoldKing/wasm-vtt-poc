use crate::entities::ping::Ping;
use crate::http::prelude::*;

#[get("")]
async fn index() -> EndpointResult {
    Ok(Response::ok(Ping { ping: "pong" }))
}
