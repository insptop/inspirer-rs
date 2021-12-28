pub mod framework;
pub mod macros;
pub mod contracts;
pub mod application;
mod error;

pub use error::{Result, Error};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
