use crate::{Error, Result};
use std::{path::Path, ops::Deref};

use crate::contracts::{
    InspirerRsApplication, InspirerRsApplicationCreator, INSPIRER_RS_APPLICATION_CREATOR,
};
use libloading::{Library, Symbol};

#[derive(Default)]
pub struct InspirerRsApplications {
    libs: Vec<Library>,
    apps: Vec<Box<dyn InspirerRsApplication>>,
}

impl InspirerRsApplications {
    pub fn load<P: AsRef<Path>>(&mut self, lib: P) -> Result<()> {
        log::debug!("Load application from {:?}", lib.as_ref());

        unsafe {
            let library = Library::new(lib.as_ref())?;
            self.libs.push(library);

            let library_ref = self.libs.last().unwrap();
            let constructor: Symbol<InspirerRsApplicationCreator> =
                library_ref.get(INSPIRER_RS_APPLICATION_CREATOR.as_bytes())?;

            let boxed_raw = constructor();
            let application = Box::from_raw(boxed_raw);

            log::info!(
                "Loaded application [{}] (from: {})",
                application.name(),
                lib.as_ref().to_string_lossy()
            );

            application.on_load().map_err(|err| {
                log::error!("Load application error: {}", err);
                err
            })?;

            self.apps.push(application)
        }

        Ok(())
    }

    pub fn destroy(&mut self) {
        log::info!("Destroy applications.");

        for application in self.apps.drain(..) {
            let name = application.name().to_string();
            log::trace!("Unload application {}", name);
            let _ = application.on_unload().map_err(|err| {
                log::error!(
                    "Fatal error on unloading, application: {}, error: {}",
                    name,
                    err
                );
                ()
            });
        }

        for library in self.libs.drain(..) {
            let _ = library.close().map_err(|err| {
                log::error!(
                    "Fatal error on close library, error: {}",
                    err
                );
                ()
            });
        }
    }
}

impl Drop for InspirerRsApplications {
    fn drop(&mut self) {
        if !self.apps.is_empty() {
            self.destroy();
        }
    }
}

impl Deref for InspirerRsApplications {
    type Target = Vec<Box<dyn InspirerRsApplication>>;

    fn deref(&self) -> &Self::Target {
        &self.apps
    }
}
