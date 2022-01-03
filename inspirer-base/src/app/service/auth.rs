use inspirer_core::application::{Service, ServiceProject};

#[derive(Clone)]
pub struct Auth {
    srv: Service
}

impl ServiceProject for Auth {
    fn construct(service: Service) -> Self {
        Auth {
            srv: service
        }
    }
}

impl Auth {
    pub async fn attepmt() {
        
    }
}