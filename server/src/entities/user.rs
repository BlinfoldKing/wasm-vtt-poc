use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(request: CreateUserRequest, password_salt: &String) -> Result<Self, Error> {
        let salt: [u8; 16] = password_salt.as_bytes().try_into().unwrap();

        let id = Uuid::new_v4();

        let password_hash = bcrypt::hash_with_salt(request.password, 12, salt)
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?
            .to_string();

        let username = request.username;

        Ok(User {
            id,
            username,
            password_hash,
        })
    }
}
