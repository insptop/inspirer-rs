use inspirer_core::{contracts::ApplicationInject, application::{ApplicationHandler, create_web_application_runtime, ApplicationRuntime}, Router, routing::get};
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

    fn run(&self) -> Result<ApplicationRuntime> {
        let router = Router::new()
            .route("/", get(hello));

        create_web_application_runtime(router)
    }
}

async fn hello() -> &'static str {
    "application report!"
}