use std::process;

use super::RunCommand;
use crate::app::App;
use crate::error::{Error, ErrorKind};
use actix_web::{web, App as HttpApp, HttpServer};
use bevy::prelude::App as Engine;

pub struct ServerRunner;

impl RunCommand<ServerRunner> for App {
    async fn run(&mut self) -> Result<(), crate::error::Error> {
        let usecase = self.usecase.clone();
        let engine = self.engine.clone();

        // setup http server
        let server = HttpServer::new(move || {
            HttpApp::new()
                .app_data(web::Data::new(usecase.clone()))
                .app_data(web::Data::new(engine.clone()))
                .configure(Self::http_services)
        })
        .bind(("0.0.0.0", self.config.http.port))
        .map_err(|err| Error::new(err.to_string(), ErrorKind::Config))?;
        let mut engine = Engine::new();

        self.configure_engine(&mut engine)?;

        let http_runner = tokio::spawn(server.run());
        let engine_runner = tokio::spawn(async move { engine.run() });

        println!("renet server run on :{}", self.config.renet.port);
        println!("http server run on :{}", self.config.http.port);

        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("exited");
                process::exit(0x100) // nuke exit; need better way to exit if exists
            }
            _ => (),
        }

        let _ = engine_runner
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Config))?;
        let _ = http_runner
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Config))?;

        Ok(())
    }
}
