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