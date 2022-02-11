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

    fn run(&self) -> ApplicationRuntime {
        let router = Router::new()
            .route("/", get(hello));

        let runtime = create_web_application_runtime(router).unwrap();
        match runtime.application_handler {
            ApplicationHandler::WebApplicationHandler(ref h) => {
                assert!(!h.service_agent.is_closed());
            }
        }

        runtime
    }
}

async fn hello() -> &'static str {
    "application report!"
}