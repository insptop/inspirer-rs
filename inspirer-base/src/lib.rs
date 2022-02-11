#[macro_use]
extern crate inspirer_core;
#[macro_use]
extern crate sea_orm;
#[macro_use]
extern crate serde;

pub(crate) mod app;

fn constructor() -> app::InspirerBaseApplication {
    app::InspirerBaseApplication::default()
}

declare_inspirer_application!(app::InspirerBaseApplication, constructor);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
