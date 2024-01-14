use crate::error::Error;
use crate::{repository::sqlite::SqliteRepository, usecases::user::UserUsecase};
use actix_web::{get, web, HttpResponse, Result};

#[get("/users")]
async fn index(usecase: web::Data<UserUsecase<SqliteRepository>>) -> Result<HttpResponse, Error> {
    let users = usecase.get_user_list()?;
    Ok(HttpResponse::Ok().json(users))
}
