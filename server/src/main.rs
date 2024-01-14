pub mod app;
pub mod cmd;
pub mod config;
pub mod entities;
pub mod error;
pub mod http;
pub mod repository;
pub mod setup;
pub mod usecases;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    cmd::run().await
}
