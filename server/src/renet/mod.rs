use bevy_renet::renet::{ConnectionConfig, RenetServer};

use crate::app::App;

impl App {
    pub fn setup_renet(&self) -> RenetServer {
        let server = RenetServer::new(ConnectionConfig::default());

        server
    }
}
