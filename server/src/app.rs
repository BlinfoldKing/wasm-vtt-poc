use std::sync::{Arc, Mutex, MutexGuard};

use crate::config::Cfg;
use crate::error::{Error, ErrorKind};
use crate::usecases::Usecase;
use bevy::app::App as Engine;
use tracing_subscriber::util::SubscriberInitExt;

pub struct App {
    pub config: Cfg,

    pub engine: Arc<Mutex<Engine>>,

    pub usecase: Arc<Mutex<Usecase>>,
}

impl App {
    pub fn new(config: Cfg) -> Result<Self, Error> {
        let max_log_level = match &*config.logging.level {
            "error" => Ok(tracing::Level::ERROR),
            "warn" => Ok(tracing::Level::WARN),
            "info" => Ok(tracing::Level::INFO),
            "debug" => Ok(tracing::Level::DEBUG),
            "trace" => Ok(tracing::Level::TRACE),
            _ => Err(Error::new(
                "unknown logging level".to_owned(),
                ErrorKind::Config,
            )),
        }?;
        tracing_subscriber::fmt()
            .with_thread_names(true)
            .with_target(true)
            .with_file(false)
            .with_line_number(true)
            .with_level(true)
            .with_max_level(max_log_level)
            .finish()
            .init();

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
