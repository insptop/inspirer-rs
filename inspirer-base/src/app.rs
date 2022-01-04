use inspirer_core::{contracts::InspirerRsApplication, Router};

pub mod controller;
pub mod dao;
pub mod model;
pub mod service;
pub mod route;

#[derive(Default)]
pub struct InspirerBaseApplication;

impl InspirerRsApplication for InspirerBaseApplication {
    fn name(&self) -> &'static str {
        "inspirer-base"
    }

    fn description(&self) -> &'static str {
        "Inspirer Base application."
    }

    fn on_load(&self) -> inspirer_core::Result<()> {
        Ok(())
    }

    fn on_unload(&self) -> inspirer_core::Result<()> {
        Ok(())
    }

    fn get_routes(&self) -> Option<Router> {
        Some(route::get_routes())
    }
}