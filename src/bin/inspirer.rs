use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use axum::Router;
use clap::{Parser, Subcommand};
use inspirer_foundation::{
    component::config::{File, FileSourceFile},
    Error,
};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Start {
        #[clap(long, short)]
        daemon: Option<bool>,
        #[clap(long, short)]
        config: Option<PathBuf>,
    },
    Stop,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    dotenv::dotenv().ok();
    env_logger::init();

    match cli.command {
        Subcommands::Start { daemon, config } => start(daemon.unwrap_or_default(), config)?,
        Subcommands::Stop => (),
    }

    Ok(())
}

fn start(daemon: bool, config: Option<PathBuf>) -> Result<()> {
    use inspirer_rs::server::start as start_server_with_default_rt;

    start_server_with_default_rt(File::<FileSourceFile>::from(
        config.ok_or(Error::GetConfigurationFailedError)?,
    ))?;

    Ok(())
}
