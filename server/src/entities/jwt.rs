use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::user::User;
use crate::error::{Error, ErrorKind};

pub struct JWTToken(String);

#[derive(Serialize, Deserialize)]
struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub user: User,
}

impl Claims {
    pub fn new(user: User) -> Result<Self, Error> {
        let now = Local::now();
        let iat = now.timestamp() as u64;
        let exp = (now + Duration::hours(12)).timestamp() as u64;

        Ok(Self { iat, exp, user })
    }
}

impl JWTToken {
    pub fn new(secret: String, user: User) -> Result<Self, Error> {
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &Claims::new(user)?,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(Self(token))
    }

    pub fn verify(&self, secret: String) -> Result<User, Error> {
        let token = decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(token.claims.user)
    }

    pub fn from(str: &str) -> Self {
        Self(str.to_owned())
    }

    pub fn get(&self) -> String {
        self.0.clone()
    }
}
