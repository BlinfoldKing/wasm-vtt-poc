use std::sync::{Arc, Mutex, MutexGuard};

use crate::config::Cfg;
use crate::error::{Error, ErrorKind};
use crate::usecases::Usecase;
use bevy::app::App as Engine;

pub struct App {
    pub config: Cfg,

    pub engine: Arc<Mutex<Engine>>,

    pub usecase: Arc<Mutex<Usecase>>,
}

impl App {
    pub fn new(config: Cfg) -> Result<Self, Error> {
        let engine = Engine::new();
        let usecase = Usecase::new(config.clone())?;

        Ok(Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
            usecase: Arc::new(Mutex::new(usecase)),
        })
    }

    pub fn get_usecase(&self) -> Result<MutexGuard<'_, Usecase>, Error> {
        let usecase = self
            .usecase
            .lock()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(usecase)
    }
}
