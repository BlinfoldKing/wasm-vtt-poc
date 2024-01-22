use clap::{Parser, Subcommand};
use crate::app::App;
use crate::config::Cfg;
use crate::error::Error;

use self::server::ServerRunner;
use self::setup::SetupRunner;

pub mod setup;
pub mod server;

#[derive(Parser)]
#[command(
    author, 
    version = env!("CARGO_PKG_VERSION"),
    about = "a (mostly) hackable and (hopefully) game agnostic virtual table top",
    long_about = None
)]

pub struct Cmd {
    /// Turn debugging information on
    #[arg(short, long)]
    config_file: Option<String>,


    #[command(subcommand)]
    command: Option<Commands>,
}

pub trait RunCommand<T> {
    #[allow(async_fn_in_trait)]
    async fn run(&mut self) -> Result<(), Error>;
}

#[derive(Subcommand)]
pub enum Commands {
    // run renet, and http
    Serve,

    /// run the http server
    Http,

    /// run the http server
    Renet,

    /// setup the app
    Setup
}

pub async fn run() -> Result<(), Error> {
    let c = Cmd::parse();
    
    let config = Cfg::new(c.config_file.as_deref())?;
    let mut app = App::new(config)?;
   
    match c.command {
       Some(Commands::Serve) => <App as RunCommand<ServerRunner>>::run(&mut app).await,
       Some(Commands::Setup) => <App as RunCommand<SetupRunner>>::run(&mut app).await,
        _ => unimplemented!("command not found or not yet implemented")
    }

}

