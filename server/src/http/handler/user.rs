use crate::{entities::user::User, http::prelude::*};

#[get("")]
async fn index(usecase: SharedUsecase) -> EndpointResult {
    let users = usecase.lock().unwrap().user.get_user_list()?;
    Ok(Response::ok(users))
}

#[get("/me")]
async fn me(user: ReqData<User>, usecase: SharedUsecase) -> EndpointResult {
    match usecase.lock().unwrap().user.get_user_by_id(user.id)? {
        Some(user) => Ok(Response::ok(user)),
        None => Err(Error::new("user not found".to_owned(), ErrorKind::NotFound)),
    }
}
