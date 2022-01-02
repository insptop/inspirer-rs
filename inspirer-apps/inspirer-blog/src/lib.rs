#[macro_use]
extern crate sea_orm;
#[macro_use]
extern crate inspirer_core;
#[macro_use]
extern crate serde;

pub mod app;

pub use inspirer_core::{Error, Result};

fn constructor() -> app::InspirerBlogApplication {
    app::InspirerBlogApplication::default()
}

declare_inspirer_rs_application!(app::InspirerBlogApplication, constructor);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
