#[macro_use]
extern crate inspirer_core;
#[macro_use]
extern crate sea_orm;

pub(crate) mod app;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
