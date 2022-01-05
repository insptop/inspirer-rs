use inspirer_core::{Router, routing::{post}, AddExtensionLayer, application::ApplicationShared};
use super::controller::auth;

pub fn get_routes(shared: ApplicationShared) -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .layer(AddExtensionLayer::new(shared))
}