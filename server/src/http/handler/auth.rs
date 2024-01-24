use crate::{entities::auth::LoginRequest, http::prelude::*};

#[post("/login")]
async fn index(
    usecase: SharedUsecase,
    JsonRequest(req): JsonRequest<LoginRequest>,
) -> EndpointResult {
    let res = usecase.lock().unwrap().auth.login(req)?;
    Ok(Response::ok(res))
}
