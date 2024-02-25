use serde::{Deserialize, Serialize};

use super::user::{User, UserResponse};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
}
