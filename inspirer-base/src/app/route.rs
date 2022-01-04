use inspirer_core::{Router, routing::{post}};
use super::controller::auth;

pub fn get_routes() -> Router {
    Router::new()
        .route("/login", post(auth::login))
}