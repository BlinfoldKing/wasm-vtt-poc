use crate::app::App;
use crate::error::Error;
use bevy::prelude::{App as Engine, *};

pub mod plugins;

impl App {
    pub fn configure_engine(&mut self, engine: &mut Engine) -> Result<(), Error> {
        engine.add_plugins(MinimalPlugins);

        Ok(())
    }
}
