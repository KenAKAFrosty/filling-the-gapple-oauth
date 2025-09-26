#[cfg(feature = "ssr")]
use std::{ops::Deref, sync::Arc};

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct ServerConfig {
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct ServerContextData {
    pub config: ServerConfig,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct ServerContext(Arc<ServerContextData>);

#[cfg(feature = "ssr")]
impl ServerContext {
    pub fn new(data: ServerContextData) -> Self {
        Self(Arc::new(data))
    }
}

#[cfg(feature = "ssr")]
impl Deref for ServerContext {
    type Target = ServerContextData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
