use inspirer_core::{contracts::ApplicationInject, application::ApplicationHandler};
use inspirer_foundation::Result;


#[derive(Default)]
pub struct InspirerBaseApplication;

impl ApplicationInject for InspirerBaseApplication {
    fn name(&self) -> &'static str {
        "inspirer-base"
    }

    fn description(&self) -> &'static str {
        "Inspirer Base application."
    }

    fn on_load(&self) -> Result<()> {
        env_logger::init();
        Ok(())
    }

    fn on_unload(&self) -> Result<()> {
        Ok(())
    }

    fn run(&self) -> ApplicationHandler {
        ApplicationHandler
    }
}