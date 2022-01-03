use std::net::SocketAddr;

use axum::Router;

use inspirer_core::{Error, Result, config::Repository};
use tokio::runtime::Runtime;

pub async fn start_server(listen: &SocketAddr, router: Router) -> Result<()> {
    axum::Server::bind(listen)
        .serve(router.into_make_service())
        .await
        .map_err(Into::into)
}

pub fn start(listen: &SocketAddr, router: Router) -> Result<()> {
    // Create service and application shared
    

    let runtime = Runtime::new().map_err(|err| {
        log::error!("Create runtime failed: {}", err);
        Error::RuntimeBuildError(err)
    })?;

    runtime.block_on(async move {
        start_server(listen, router.clone()).await
    })
}

