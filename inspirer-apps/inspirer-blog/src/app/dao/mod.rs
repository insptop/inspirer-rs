use std::marker::PhantomData;
use sea_orm::ConnectionTrait;

pub mod content;

pub struct DaoProject<'a, C, T>
where
    C: ConnectionTrait<'a>,
{
    db: &'a C,
    _phantom: PhantomData<T>,
}

pub struct Dao<T>(PhantomData<T>);

impl<T> Dao<T> {
    pub fn on<'a, C: ConnectionTrait<'a>>(db: &'a C) -> DaoProject<'a, C, T> {
        DaoProject {
            db,
            _phantom: PhantomData,
        }
    }
}
