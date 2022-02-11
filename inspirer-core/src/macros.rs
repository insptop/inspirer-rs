#[macro_export]
macro_rules! declare_inspirer_application {
    ($application:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _inspirer_application_creator() -> *mut dyn $crate::contracts::ApplicationInject {
            let constructor: fn() -> $application = $constructor;

            let application = constructor();
            let boxed: Box<dyn $crate::contracts::ApplicationInject> = Box::new(application);
            
            Box::into_raw(boxed)
        }
    };
}