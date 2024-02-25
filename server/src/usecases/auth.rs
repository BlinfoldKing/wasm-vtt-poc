use crate::{
    config::Cfg,
    entities::{
        auth::{LoginRequest, LoginResponse},
        jwt::JWTToken,
    },
    error::{Error, ErrorKind},
    repository::user::UserRepository,
};

#[derive(Debug, Clone)]
pub struct AuthUsecase<UserRepo: UserRepository> {
    config: Cfg,
    pub user_repo: UserRepo,
}

impl<UserRepo: UserRepository> AuthUsecase<UserRepo> {
    pub fn new(config: Cfg, user_repo: UserRepo) -> Result<Self, Error> {
        Ok(Self { config, user_repo })
    }

    pub fn login(&self, req: LoginRequest) -> Result<LoginResponse, Error> {
        match self.user_repo.get_user_by_username(req.username)? {
            None => Err(Error::new("user not found".to_owned(), ErrorKind::NotFound)),
            Some(user) => {
                user.validate_password(req.password)?;
                let token = JWTToken::new(self.config.general.secret.clone(), user.clone())?;

                Ok(LoginResponse {
                    user: user.to_response(),
                    token: token.get(),
                })
            }
        }
    }
}
