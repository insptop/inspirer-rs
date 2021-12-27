#[macro_use]
extern crate sea_orm;

pub mod app;
pub mod error;

pub(crate) use error::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
