pub mod setting;
pub mod user;

use crate::config::Cfg;
use crate::error::Error;

pub mod sqlite;

pub trait Repository {
    fn new(config: &Cfg) -> Result<Box<Self>, Error>;
}
