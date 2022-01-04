use std::{path::PathBuf, env::current_dir};

use anyhow::Result;
use axum::Router;
use clap::{Parser, Subcommand};
use inspirer_core::framework::{app_manager::InspirerRsApplications, EnviromentContext};

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

    let ctx = EnviromentContext {
        daemonize: daemon,
        config_file: config,
        work_dir: current_dir()?,
    };

    let mut apps = InspirerRsApplications::default();
    apps.load("./target/debug/inspirer_base.dll")?;
    apps.load("./target/debug/inspirer_blog.dll")?;
    apps.load("./target/debug/simple_application.dll")?;

    let mut router = Router::new();

    for app in apps.iter() {
        if let Some(routes) = app.get_routes() {
            router = router.merge(routes);
        }
    }

    start_server_with_default_rt(router, ctx)?;

    Ok(())
}
