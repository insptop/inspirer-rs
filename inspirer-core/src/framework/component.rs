use crate::{Error, Result};
use std::{sync::{Arc, Mutex}, future::Future};
use type_map::TypeMap;

pub mod environment;
pub mod config;
pub mod database;

pub struct ComponentManager {
    inner: Arc<Mutex<ComponentManagerInner>>,
}

impl ComponentManager {
    pub async fn register<T, R, C>(&self, provider: T) -> Result<()>
    where
        T: Fn(&ComponentManager) -> R,
        R: Future<Output = Result<C>>,
        C: Clone + 'static,
    {
        let mut inner = self.inner.lock().or_else(|err| {
            log::error!("Register component error: {}", err);
            Err(Error::ComponentManagerError)
        })?;

        inner.register(provider(&self).await?);

        Ok(())
    }

    pub fn component<T: Clone + 'static>(&self) -> Result<Option<T>> {
        let inner = self.inner.lock().or_else(|err| {
            log::error!("Register component error: {}", err);
            Err(Error::ComponentManagerError)
        })?;

        Ok(inner.get::<T>().cloned())
    }
}

pub struct ComponentManagerInner {
    components: TypeMap,
}

impl ComponentManagerInner {
    pub fn register<T: Clone + 'static>(&mut self, component: T) {
        self.components.insert(component);
    }

    pub fn get<T: Clone + 'static>(&self) -> Option<&T> {
        self.components.get::<T>()
    }
}
