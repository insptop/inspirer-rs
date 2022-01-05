use std::any::Any;

use axum::Router;

use crate::{Result, application::ApplicationShared};

pub const INSPIRER_RS_APPLICATION_CREATOR: &'static str = "inspirer_rs_application_creator";

pub type InspirerRsApplicationCreator = unsafe fn() -> *mut dyn InspirerRsApplication;

pub trait InspirerRsApplication: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn on_load(&self) -> Result<()>;
    fn on_unload(&self) -> Result<()>;
    fn get_routes(&self, shared: ApplicationShared) -> Option<Router>;
}