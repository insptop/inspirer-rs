use std::cell::RefCell;

use axum::Router;

#[derive(Default)]
pub struct ApplicationFramework {
    router: RefCell<Option<Router>>
}

impl ApplicationFramework {
    pub fn register_router(&self, router: Router) {
        let mut inner = self.router.borrow_mut();
        if inner.is_some() {
            let before = inner.take().unwrap();
            inner.replace(before.merge(router));
        } else {
            inner.replace(router);
        }
    }
}