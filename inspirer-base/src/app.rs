use inspirer_core::{contracts::{InspirerRsApplicationInject, InspirerRsApplication}, Router, application::ApplicationShared};

pub mod controller;
pub mod dao;
pub mod model;
pub mod service;
pub mod route;

#[derive(Default)]
pub struct InspirerBaseApplication;

impl InspirerRsApplicationInject for InspirerBaseApplication {
    fn name(&self) -> &'static str {
        "inspirer-base"
    }

    fn description(&self) -> &'static str {
        "Inspirer Base application."
    }

    fn on_load(&self) -> inspirer_core::Result<()> {
        env_logger::init();
        Ok(())
    }

    fn on_unload(&self) -> inspirer_core::Result<()> {
        Ok(())
    }

    fn get_routes(&self, shared: ApplicationShared) -> Option<Router> {
        Some(route::get_routes(shared))
    }

    fn get_application_constructor(&self, ctx: inspirer_core::contracts::RuntimeContext) -> Box<dyn inspirer_core::contracts::InspirerRsApplication> {
        Box::new(Application {
            handle: ctx.handle.clone()
        })
    }
}


pub struct Application {
    handle: inspirer_core::contracts::Handle,
}

impl InspirerRsApplication for Application {

}
