use crate::{config::Cfg, error::Error, repository::sqlite::SqliteRepository};

use self::{setting::SettingUsecase, user::UserUsecase};

pub mod setting;
pub mod user;

pub struct Usecase {
    // TODO: make this usecases shared a sqlite instance
    pub user: UserUsecase<SqliteRepository>,
    pub setting: SettingUsecase<SqliteRepository>,
}

impl Usecase {
    pub fn new(config: Cfg) -> Result<Self, Error> {
        let user = UserUsecase::new(config.clone())?;
        let setting = SettingUsecase::new(config.clone())?;

        Ok(Self { user, setting })
    }
}
