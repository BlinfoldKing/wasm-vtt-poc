use crate::error::{Error, ErrorKind};
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Cfg {
    pub general: General,
    pub http: Http,
    pub renet: Renet,
    pub database: Database,
    pub logging: Logging,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub host: String,
    pub password_salt: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Http {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Renet {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub filepath: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logging {
    pub level: String,
}

impl Cfg {
    pub fn new(config_file: Option<&str>) -> Result<Self, Error> {
        let c = Config::builder()
            .add_source(File::with_name(config_file.unwrap_or("ouroboros")))
            .add_source(Environment::with_prefix("ouroboros"))
            .build()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Config))?;

        c.try_deserialize()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Config))
    }
}
