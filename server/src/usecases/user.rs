use crate::{
    config::Cfg,
    entities::user::{CreateUserRequest, User},
    error::Error,
    repository::user::UserRepository,
};

#[derive(Debug, Clone)]
pub struct UserUsecase<T: UserRepository> {
    config: Cfg,
    pub repo: T,
}

impl<T: UserRepository> UserUsecase<T> {
    pub fn new(config: Cfg, repo: T) -> Result<Self, Error> {
        Ok(Self { config, repo })
    }

    pub fn create_user(&self, request: CreateUserRequest) -> Result<User, Error> {
        let user = User::new(request, &self.config.general.password_salt)?;

        self.repo.save_user(user)
    }

    pub fn get_user_list(&self) -> Result<Vec<User>, Error> {
        return self.repo.get_users();
    }

    pub fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, Error> {
        return self.repo.get_user_by_id(id);
    }
}
