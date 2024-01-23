use std::{net::UdpSocket, time::SystemTime};

use crate::config::Renet as RenetConfig;
use bevy::prelude::*;
use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};

pub struct NetworkPlugin(pub RenetConfig);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let public_address = format!("0.0.0.0:{}", self.0.port).parse().unwrap();
        let socket = UdpSocket::bind(public_address).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let server_config = ServerConfig {
            current_time,
            public_addresses: vec![public_address],
            protocol_id: 0,
            max_clients: 64,
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let server = RenetServer::new(ConnectionConfig::default());

        app.insert_resource(server);
        app.insert_resource(transport);
    }
}
