use std::any::Any;
use inspirer_foundation::Result;
use crate::application::ApplicationHandler;

pub const APPLICATION_CREATOR: &'static str = "_inspirer_application_creator";

pub type ApplicationCreator = unsafe fn() -> *mut dyn ApplicationInject;

pub trait ApplicationInject: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn on_load(&self) -> Result<()>;
    fn on_unload(&self) -> Result<()>;
    fn run(&self) -> ApplicationHandler;
}