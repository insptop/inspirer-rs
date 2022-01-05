use axum::extract::Path;
use axum::{routing::get, Router, AddExtensionLayer};
use inspirer_core::application::ApplicationShared;
use inspirer_core::contracts::InspirerRsApplication;
use inspirer_core::declare_inspirer_rs_application;
use inspirer_core::Result;

fn simple_application_constrcutor() -> SimpleApp {
    SimpleApp::default()
}

#[derive(Default)]
pub struct SimpleApp;

async fn test() -> &'static str {
    "test"
}

async fn get_context(shared: ApplicationShared) -> Result<String> {
    shared
        .service
        .config
        .get::<String>("server.listen")
        .map(|res| res.unwrap_or("none".to_string()))
}

async fn path_param(Path((id, )): Path<(i64, )>) -> String {
    format!("received: {}", id)
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

    fn get_routes(&self, shared: ApplicationShared) -> Option<Router> {
        Some(
            Router::new()
                .route("/simple/path/:id", get(path_param))
                .route("/simple/test", get(test))
                .route("/simple/config", get(get_context))
                .layer(AddExtensionLayer::new(shared)),
        )
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
