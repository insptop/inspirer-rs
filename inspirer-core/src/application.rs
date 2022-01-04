use axum::extract::{FromRequest, RequestParts};
use sea_orm::{ConnectionTrait, DatabaseConnection};

use crate::{config::Repository, Error, framework::component::ComponentManager};

pub struct DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    connection: &'a C,
}

impl<'a, C> DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    pub fn connection(&self) -> &'a C {
        self.connection
    }
}

pub struct Dao;

impl Dao {
    pub fn new<'a, C: ConnectionTrait<'a>>(connection: &'a C) -> DaoProject<'a, C> {
        DaoProject { connection }
    }
}

#[derive(Clone)]
pub struct Service {
    pub database: DatabaseConnection,
    pub config: Repository,
}

impl Service {
    pub fn project<T: ServiceProject>(&self) -> T {
        T::construct(self.clone())
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.database
    }

    pub fn config(&self) -> &Repository {
        &self.config
    }
}

pub trait ServiceProject {
    fn construct(service: Service) -> Self;
}

#[derive(Clone)]
pub struct ApplicationShared {
    pub service: Service,
}

#[async_trait]
impl<B> FromRequest<B> for ApplicationShared
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        log::debug!("Extract application shared.");
        let res = req.extensions()
            .ok_or_else(|| {
                log::error!("<Application> component extract error");
                Error::ExtractApplicationComponentError
            })?
            .get::<ApplicationShared>()
            .ok_or_else(|| {
                log::error!("<Application> component is not found!");
                Error::ExtractApplicationComponentError
            })
            .map(|res| res.clone())?;

        Ok(res)
        
    }
}
