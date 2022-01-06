#[macro_export]
macro_rules! declare_inspirer_rs_application {
    ($application:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn inspirer_rs_application_creator() -> *mut dyn $crate::contracts::InspirerRsApplicationInject {
            let constructor: fn() -> $application = $constructor;

            let application = constructor();
            let boxed: Box<dyn $crate::contracts::InspirerRsApplicationInject> = Box::new(application);
            
            Box::into_raw(boxed)
        }
    };
}