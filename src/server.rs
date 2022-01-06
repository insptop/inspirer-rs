use std::{net::SocketAddr, path::PathBuf};

use axum::{Router, AddExtensionLayer};

use inspirer_core::{
    application::ApplicationShared,
    config::Repository,
    framework::{
        component::{self, ComponentManager},
        EnviromentContext, app_manager::InspirerRsApplications,
    },
    Error, Result,
};
use serde::Deserialize;
use tokio::runtime::Runtime;

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

pub fn start(ctx: EnviromentContext) -> Result<()> {
    let runtime = Runtime::new().map_err(|err| {
        log::error!("Create runtime failed: {}", err);
        Error::RuntimeBuildError(err)
    })?;

    runtime.block_on(async move {
        log::debug!("Start async runtime.");
        // Create kernel
        let cm = ComponentManager::default();

        cm.register(component::environment::component_constructor_builder(ctx))
            .await?;
        cm.register(component::config::component_constructor)
            .await?;
        cm.register(component::database::component_constructor)
            .await?;

    
        let config = cm.component::<Repository>()?;
        let server_config = config
            .unwrap()
            .get::<ServerConfig>("server")?
            .unwrap_or_default();

        log::debug!("Server config: {:?}", server_config);

        let shared = ApplicationShared {
            service: inspirer_core::application::Service {
                database: cm.component()?.unwrap(),
                config: cm.component()?.unwrap(),
            },
        };


        let mut apps = InspirerRsApplications::default();


        for lib in server_config.apps.iter() {
            apps.load(lib)?;
        }
        
        let mut router = Router::new();

        for app in apps.iter() {
            if let Some(routes) = app.get_routes(shared.clone()) {
                router = router.merge(routes);
            }
        }

        let router = router.layer(AddExtensionLayer::new(shared));

        start_server(&server_config.listen, router).await
    })
}
