use inspirer_core::contracts::InspirerRsApplication;
use inspirer_core::{Router, routing::get};

pub(crate) mod model;
pub(crate) mod dao;
pub(crate) mod service;
pub(crate) mod controller;

#[derive(Default)]
pub struct InspirerBlogApplication;

impl InspirerRsApplication for InspirerBlogApplication {
    fn name(&self) -> &'static str {
        "inspirer-blog"
    }

    fn description(&self) -> &'static str {
        "Inspirer Blog application."
    }

    fn on_load(&self) -> inspirer_core::Result<()> {
        Ok(())
    }

    fn on_unload(&self) -> inspirer_core::Result<()> {
        Ok(())
    }

    fn get_routes(&self) -> Option<Router> {
        Some(Router::new().route("/content", get(controller::content::get_content_list)))
    }
}