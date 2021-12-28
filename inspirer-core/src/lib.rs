pub mod framework;
mod application;
mod error;

pub use inspirer_common::{Result, Error};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
