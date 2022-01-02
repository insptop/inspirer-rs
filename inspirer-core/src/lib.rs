pub mod framework;
pub mod macros;
pub mod contracts;
pub mod application;
pub mod dao;
mod error;

pub use error::{Result, Error};
pub use async_trait::async_trait;
pub use axum::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
