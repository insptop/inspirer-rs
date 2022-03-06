use std::{net::SocketAddr, path::PathBuf};

use axum::routing::any_service;
use axum::{Router, AddExtensionLayer};

use inspirer_core::application::ApplicationHandler;
use inspirer_foundation::{component::*, Result, Error};
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
    // todo
    
    Ok(())
}