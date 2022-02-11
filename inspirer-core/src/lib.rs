#[macro_use]
extern crate async_trait;

pub mod macros;
pub mod contracts;
pub mod application;

pub use async_trait::async_trait;
pub use axum::*;
pub use log::{self, debug, trace, info, warn, error};
pub use axum::http::StatusCode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
