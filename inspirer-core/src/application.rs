use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Arc;

use axum::{response::Response, Router};
use hyper::{Body, Request};
use inspirer_foundation::{Result, Error};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tower::Service;

/// 数据交换对象
pub struct ApplicationRuntime {
    pub handle: std::thread::JoinHandle<()>,
    pub application_handler: ApplicationHandler
}

pub enum ApplicationHandler {
    WebApplicationHandler(WebApplicationHandler)
}

#[derive(Clone)]
pub struct WebApplicationHandler {
    pub service_agent: mpsc::UnboundedSender<(Request<Body>, oneshot::Sender<ServiceResult>)>,
}

type ServiceResult = std::result::Result<Response, Infallible>;

pub fn create_web_application_runtime(app: Router) -> Result<ApplicationRuntime> {
    let (sender, mut receiver) =
        mpsc::unbounded_channel::<(Request<Body>, oneshot::Sender<ServiceResult>)>();

    let handle = std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            let mut app = app;
            loop {
                match receiver.recv().await {
                    Some((request, response_sender)) => {
                        log::debug!("Extension application received request: {}", request.uri());
                        match response_sender.send(app.call(request).await) {
                            Ok(_) => (),
                            Err(_err) => {
                                break;
                            }
                        }
                    }
                    None => break,
                }
            }
    
            log::warn!("Application shutdown!");
        });
    });

    assert!(!sender.is_closed());

    Ok(ApplicationRuntime {
        handle,
        application_handler: ApplicationHandler::WebApplicationHandler(WebApplicationHandler {
            service_agent: sender
        })
    })
}

impl Service<Request<Body>> for WebApplicationHandler {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = ServiceResult> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let sender = self.service_agent.clone();
        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            sender.send((req, tx)).unwrap();

            rx.await.unwrap()
        })
    }
}