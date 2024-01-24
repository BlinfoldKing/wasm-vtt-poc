use crate::{
    config::Cfg,
    error::Error,
    repository::{sqlite::SqliteRepository, Repository},
};

use self::{auth::AuthUsecase, setting::SettingUsecase, user::UserUsecase};

pub mod auth;
pub mod setting;
pub mod user;

pub struct Usecase {
    // TODO: make this usecases shared a sqlite instance
    pub auth: AuthUsecase<SqliteRepository>,
    pub user: UserUsecase<SqliteRepository>,
    pub setting: SettingUsecase<SqliteRepository>,
}

impl Usecase {
    pub fn new(config: Cfg) -> Result<Self, Error> {
        let repo = *SqliteRepository::new(&config)?;

        let auth = AuthUsecase::new(config.clone(), repo.clone())?;
        let user = UserUsecase::new(config.clone(), repo.clone())?;
        let setting = SettingUsecase::new(config.clone(), repo.clone())?;

        Ok(Self {
            auth,
            user,
            setting,
        })
    }
}
