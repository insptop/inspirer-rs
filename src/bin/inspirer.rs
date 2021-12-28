use inspirer_core::framework::app_manager::InspirerRsApplications;
use anyhow::Result;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let mut apps = InspirerRsApplications::default();

    apps.load("./target/debug/simple_application.dll")?;

    println!("exit");
    
    Ok(())
}