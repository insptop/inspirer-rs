use crate::{Error, Result};
use std::{sync::{Arc, Mutex}, future::Future, any::type_name};
use type_map::TypeMap;

pub mod environment;
pub mod config;
pub mod database;

#[derive(Default)]
pub struct ComponentManager {
    inner: Arc<Mutex<ComponentManagerInner>>,
}

impl ComponentManager {
    pub async fn register<'r, T, R, C>(&'r self, provider: T) -> Result<()>
    where
        T: FnOnce(&'r ComponentManager) -> R,
        R: Future<Output = Result<C>>,
        C: Clone + 'static,
    {
        log::debug!("Register component <{}> from provider.", type_name::<C>());
        let component = provider(&self).await?;

        {
            let mut inner = self.inner.lock().or_else(|err| {
                log::error!("Register component error: {}", err);
                Err(Error::ComponentManagerError)
            })?;
    
            inner.register(component);
        }

        Ok(())
    }

    pub fn component<T: Clone + 'static>(&self) -> Result<Option<T>> {
        log::debug!("Get component <{}> from manager.", type_name::<T>());

        let res = {
            let inner = self.inner.lock().or_else(|err| {
                log::error!("Register component error: {}", err);
                Err(Error::ComponentManagerError)
            })?;

            inner.get::<T>().cloned()
        };
        

        Ok(res)
    }
}

#[derive(Default)]
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
