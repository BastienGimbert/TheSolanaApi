use std::sync::Arc;
use std::time::Duration;

use awc::Client;

use crate::validators::ValidatorRegistry;

#[derive(Clone)]
pub struct AppState {
    registry: Arc<ValidatorRegistry>,
    request_timeout: Duration,
}

impl AppState {
    pub fn new(registry: ValidatorRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
            request_timeout: Duration::from_secs(15),
        }
    }

    pub fn registry(&self) -> &ValidatorRegistry {
        self.registry.as_ref()
    }

    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }

    pub fn build_client(&self) -> Client {
        Client::builder()
            .timeout(self.request_timeout)
            .finish()
    }
}
