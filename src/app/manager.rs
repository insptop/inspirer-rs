use inspirer_core::contracts::{ApplicationInject, ApplicationCreator, APPLICATION_CREATOR};
use inspirer_foundation::Result;
use libloading::{Library, Symbol};
use std::{path::Path, ops::Deref};

#[derive(Default)]
pub struct ApplicationExtension {
    libs: Vec<Library>,
    apps: Vec<Box<dyn ApplicationInject>>,
}

impl ApplicationExtension {
    pub fn load<P: AsRef<Path>>(&mut self, lib: P) -> Result<()> {
        log::debug!("Load application from {:?}", lib.as_ref());

        unsafe {
            let library = Library::new(lib.as_ref())?;
            self.libs.push(library);

            // 此处写法并不多余，不能在 push 前就调用获取符号的逻辑
            // 否则无法正确获取动态库
            let library_ref = self.libs.last().unwrap();
            let constructor: Symbol<ApplicationCreator> =
                library_ref.get(APPLICATION_CREATOR.as_bytes())?;

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

impl Drop for ApplicationExtension {
    fn drop(&mut self) {
        if !self.apps.is_empty() {
            self.destroy();
        }
    }
}

impl Deref for ApplicationExtension {
    type Target = Vec<Box<dyn ApplicationInject>>;

    fn deref(&self) -> &Self::Target {
        &self.apps
    }
}
