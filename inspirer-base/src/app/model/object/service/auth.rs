use chrono::{Duration, Utc};

use crate::app::model::object::request::auth::LoginRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

impl From<LoginRequest> for Credential {
    fn from(LoginRequest { username, password }: LoginRequest) -> Self {
        Credential { username, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub uid: u64,
}

impl Claims {
    pub fn create(uid: u64) -> Self {
        Claims {
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            uid,
        }
    }
}
