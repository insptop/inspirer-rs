use std::sync::Arc;

use crate::{Error, Result};
use config::{Config, ConfigError};
pub use config::{Environment, File, Source, Value};
use crossbeam_utils::sync::ShardedLock;
use serde::Deserialize;

#[derive(Default)]
pub struct Repository {
    inner: Arc<ShardedLock<Config>>,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Repository {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Repository {
    pub fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let config = self.inner.read().map_err(|err| {
            log::warn!("Failed to read config: {}, error: {}", key, err);
            Error::ReadConfigError
        })?;

        match config.get(key) {
            Ok(res) => Ok(Some(res)),
            Err(ConfigError::NotFound(_)) => Ok(None),
            Err(err) => Err(Into::into(err))
        }
    }

    pub fn load<T>(&self, source: T) -> Result<()>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        let mut config = self.inner.write().map_err(|err| {
            log::warn!("Failed to load config, error: {}", err);
            Error::LoadOrWriteConfigError
        })?;

        config.merge(source)?;

        Ok(())
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value>,
    {
        let mut config = self.inner.write().map_err(|err| {
            log::warn!("Failed to write config: {}, error: {}", key, err);
            Error::LoadOrWriteConfigError
        })?;

        config.set(key, value)?;
        Ok(())
    }
}
