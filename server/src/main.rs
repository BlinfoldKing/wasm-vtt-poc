pub mod app;
pub mod cmd;
pub mod config;
pub mod engine;
pub mod entities;
pub mod error;
pub mod http;
pub mod renet;
pub mod repository;
pub mod usecases;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    cmd::run().await
}
