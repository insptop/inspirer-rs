use sea_orm::ConnectionTrait;

pub struct DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    connection: &'a C,
}

impl<'a, C> DaoProject<'a, C> 
    where  C: ConnectionTrait<'a>
{
    pub fn connection(&self) -> &'a C {
        self.connection
    }
}

#[derive(Debug, Clone)]
pub struct Service {

}

impl Service {
    pub fn project<T: ServiceProject>(&self) -> T {
        T::construct(self.clone())
    }
}

pub trait ServiceProject {
    fn construct(service: Service) -> Self;
}

