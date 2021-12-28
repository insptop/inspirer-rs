use axum::{Router, routing::get};
use inspirer_core::application::ApplicationFramework;
use inspirer_core::contracts::InspirerRsApplication;
use inspirer_core::declare_inspirer_rs_application;
use inspirer_core::Result;

fn simple_application_constrcutor() -> SimpleApp  {
    SimpleApp::default()
}

#[derive(Default)]
pub struct SimpleApp;

async fn test() -> &'static str {
    "test"
}

impl InspirerRsApplication for SimpleApp {
    fn name(&self) -> &'static str {
        "simple-application"
    }

    fn description(&self) -> &'static str {
        "This is a example application"
    }

    fn on_load(&self) -> Result<()> {
        println!("Wow, im loaded");
        Ok(())
    }

    fn on_unload(&self) -> Result<()> {
        println!("Wow, im unloaded");
        Ok(())
    }

    fn register(&self, framework: &ApplicationFramework) {
        framework.register_router(Router::new().route("/simple/test", get(test)));
    }
}

declare_inspirer_rs_application!(SimpleApp, simple_application_constrcutor);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
