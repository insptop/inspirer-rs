use std::{net::SocketAddr, path::PathBuf};

use axum::routing::any_service;
use axum::{Router, AddExtensionLayer};

use inspirer_core::application::ApplicationHandler;
use inspirer_foundation::{component, service::ServiceBuilder, Result, Error};
use inspirer_foundation::component::config::{Source, ConfigAdapter};
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::app::manager::ApplicationExtension;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen: SocketAddr,
    pub apps: Vec<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let addr = std::env::var("INSPIRER_LISTEN").unwrap_or("127.0.0.1:3008".into());

        ServerConfig {
            listen: addr.as_str().parse().expect("Format listen address error."),
            apps: Default::default(),
        }
    }
}

pub async fn start_server(listen: &SocketAddr, router: Router) -> Result<()> {
    axum::Server::bind(listen)
        .serve(router.into_make_service())
        .await
        .map_err(Into::into)
}

pub fn start<T>(config_source: T) -> Result<()> 
where T: Source + Send + Sync + 'static
{
    let runtime = Runtime::new().map_err(|err| {
        log::error!("Create runtime failed: {}", err);
        Error::RuntimeBuildError(err)
    })?;

    runtime.block_on(async move {
        log::debug!("Start async runtime.");
        // Create kernel service
        let mut service_builder = ServiceBuilder::default();
        service_builder.provide(component::config::ConfigComponentConstructor(config_source));
        service_builder.provide(component::database::DatabaseComponentConstructor);

        let service = service_builder.build().await?;
        let server_config = service.get::<ServerConfig>("server").await?;

        let mut apps = ApplicationExtension::default();

        for lib in server_config.apps.iter() {
            // 加载扩展应用
            apps.load(lib)?;
        }
        
        let mut router = Router::new();
        let mut runtimes = vec![];
        for app in apps.iter() {
            let runtime = app.run()?;

            match runtime.application_handler {
                ApplicationHandler::WebApplicationHandler(ref handler) => {
                    let handler = handler.clone();

                    log::info!("Append web application [{}] route", app.name());
                    router = router.nest(format!("/{}", app.name()).as_str(), any_service(handler));
                }
            }

            runtimes.push(runtime);
        }

        let router = router.layer(AddExtensionLayer::new(service));

        start_server(&server_config.listen, router).await
    })
}