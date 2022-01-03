use inspirer_core::{application::ApplicationShared, Json, Result};

use crate::app::{
    model::object::{request::auth::LoginRequest, response::auth::LoginSuccess},
    service::auth::Auth,
};

pub async fn login(
    Json(credential): Json<LoginRequest>,
    app: ApplicationShared,
) -> Result<Json<LoginSuccess>> {
    app.service
        .project::<Auth>()
        .login(credential.into())
        .await
        .map(|token| Json(LoginSuccess { token }))
}
