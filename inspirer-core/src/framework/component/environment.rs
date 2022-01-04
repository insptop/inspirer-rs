use super::ComponentManager;
use crate::Result;
use std::{future::{Future, Ready}, path::PathBuf};

#[derive(Clone, Debug)]
pub struct EnviromentContext {
    pub config_file: Option<PathBuf>,
    pub work_dir: PathBuf,
    pub daemonize: bool,
}

pub fn component_constructor_builder(
    ctx: EnviromentContext,
) -> Box<dyn FnOnce(&ComponentManager) -> Ready<Result<EnviromentContext>>> {
    Box::new(move |_| std::future::ready(Ok(ctx)))
}
