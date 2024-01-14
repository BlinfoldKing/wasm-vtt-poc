use clap::{Parser, Subcommand};
use crate::app::App;
use crate::config::Cfg;
use crate::error::Error;

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

#[derive(Subcommand)]
pub enum Commands {
    /// run the http server
    Http,
    /// setup the app
    Setup
}

pub async fn run() -> Result<(), Error> {
    let c = Cmd::parse();
    
    let config = Cfg::new(c.config_file.as_deref())?;
    let app =    App{ config };
   
    match c.command {
        Some(Commands::Http ) => app.serve().await,
        Some(Commands::Setup) => app.setup(),
        _ => Ok(())
    }

}

