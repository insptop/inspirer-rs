use inspirer_core::application::{Dao, Service, ServiceProject};
use inspirer_core::{log, BizError, Error, Result, StatusCode};
use jsonwebtoken::{EncodingKey, Header};

use crate::app::dao::user::InspirerBaseUserTrait;
use crate::app::model::object::service::auth::{Claims, Credential};
use crate::app::model::user;

#[derive(Clone)]
pub struct Auth {
    srv: Service,
}

impl ServiceProject for Auth {
    fn construct(service: Service) -> Self {
        Auth { srv: service }
    }
}

impl Auth {
    pub async fn attepmt(&self, credential: Credential) -> Result<(bool, user::Model)> {
        let user = Dao::new(self.srv.connection())
            .find_user_by_username(credential.username.as_str())
            .await?
            .ok_or(BizError::create_report(
                "10001",
                "用户不存在或密码错误",
                StatusCode::BAD_REQUEST,
            ))?;

        if pwhash::bcrypt::verify(&credential.password, &user.password) {
            Ok((true, user))
        } else {
            Ok((false, user))
        }
    }

    pub async fn login(&self, credential: Credential) -> Result<String> {
        log::debug!("Received login request: {:?}", credential);
        
        let (verified, user) = self.attepmt(credential).await?;

        if verified {
            let claims = Claims::create(user.id);
            let secret = self
                .srv
                .config()
                .get::<String>("base.secret")?
                .ok_or(Error::GetConfigurationFailedError)?;

            jsonwebtoken::encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .map_err(|err| {
                log::warn!("Create json web token error: {}", err);
                Error::UnknownError
            })
        } else {
            Err(BizError::create_report(
                "10002",
                "用户不存在或密码错误",
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}
