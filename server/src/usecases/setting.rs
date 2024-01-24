use crate::{
    config::Cfg, entities::setting::Setting, error::Error, repository::setting::SettingRepository,
};

#[derive(Debug, Clone)]
pub struct SettingUsecase<T: SettingRepository> {
    #[allow(unused)]
    config: Cfg,
    pub repo: T,
}

impl<T: SettingRepository> SettingUsecase<T> {
    pub fn new(config: Cfg, repo: T) -> Result<Self, Error> {
        Ok(Self { config, repo })
    }

    pub fn set_setting(&self, key: String, value: String) -> Result<Setting, Error> {
        let setting = Setting::new(key, value);
        self.repo.set_setting(setting)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<Setting>, Error> {
        self.repo.get_setting(key)
    }
}
