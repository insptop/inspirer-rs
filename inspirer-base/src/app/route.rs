use inspirer_core::{Router, routing::{get}};
use super::controller::auth;

pub fn get_routes() -> Router {
    Router::new()
        .route("/login", get(auth::login))
}