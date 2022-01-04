use config::{FileSourceFile, File};

use crate::{framework::component::environment::EnviromentContext, config::Repository, Result};

use super::ComponentManager;

pub async fn component_constructor(manager: &ComponentManager) -> Result<Repository> {
    log::debug!("Component <ConfigurationRepository> creating.");

    let repository = Repository::default();
    
    if let Some(env) = manager.component::<EnviromentContext>()? {
        log::debug!("Environment context debug {:?}", env);
        if let Some(path) = env.config_file {
            log::debug!("Load configuration from [{:?}]", path);

            repository.load(File::<FileSourceFile>::from(path.as_path()))?;

            log::debug!("Load configuration from [{:?}] successfully.", path);
        }
    } else {
        log::warn!("Environment Context component is not found !");
    }
    
    Ok(repository)
}