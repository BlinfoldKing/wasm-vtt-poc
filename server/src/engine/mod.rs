use std::sync::{Arc, Mutex};

use crate::error::Error;
use crate::{app::App, usecases::Usecase};
use bevy::prelude::{App as Engine, *};

use self::plugins::network::NetworkPlugin;

pub mod plugins;

#[derive(Resource)]
pub struct UseCaseResource(Arc<Mutex<Usecase>>);

impl App {
    pub fn configure_engine(&mut self, engine: &mut Engine) -> Result<(), Error> {
        engine
            .add_plugins((MinimalPlugins, NetworkPlugin(self.config.renet.clone())))
            .insert_resource(UseCaseResource(self.usecase.clone()));

        Ok(())
    }
}
