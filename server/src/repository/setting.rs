use crate::entities::setting::Setting;
use crate::error::Error;

use super::Repository;

pub trait SettingRepository
where
    Self: Repository,
{
    fn set_setting(&self, setting: Setting) -> Result<Setting, Error>;
    fn get_setting(&self, key: &str) -> Result<Option<Setting>, Error>;
}
