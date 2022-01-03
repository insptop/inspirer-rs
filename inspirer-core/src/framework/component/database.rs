use sea_orm::{Database, DatabaseConnection};

use crate::config::Repository;
use crate::{Error, Result};
use std::env;

use super::ComponentManager;

pub async fn component_constructor(manager: &ComponentManager) -> Result<DatabaseConnection> {
    log::trace!("Component <DatabaseConnection> creating.");

    let database_url = match manager.component::<Repository>()? {
        Some(config) => config
            .get::<String>("app.database.connection_url")
            .and_then(|res| Ok(res.or(env::var("DATABASE_URL").ok())))?,
        None => env::var("DATABASE_URL").ok(),
    };

    if let Some(database_url) = database_url {
        create_connection(database_url.as_str()).await
    } else {
        log::error!("Get connection configuration field failed.");
        Err(Error::GetConfigurationFailedError)
    }
}

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection> {
    Database::connect(database_url).await.map_err(Into::into)
}
