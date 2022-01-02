use axum::Router;
use inspirer_core::framework::app_manager::InspirerRsApplications;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommands
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Start,
    Stop
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    dotenv::dotenv().ok();
    env_logger::init();

    match cli.command {
        Subcommands::Start => start()?,
        Subcommands::Stop => ()
    }
    
    Ok(())
}

fn start() -> Result<()> {
    use inspirer_rs::server::start as start_server_with_default_rt;

    let mut apps = InspirerRsApplications::default();
    apps.load("./target/debug/simple_application.dll")?;

    let mut router = Router::new();

    for app in apps.iter() {
        if let Some(routes) = app.get_routes() {
            router = router.merge(routes);
        }
    }

    start_server_with_default_rt(&"0.0.0.0:3008".parse()?, router)?;

    Ok(())
}
