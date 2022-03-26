use std::sync::Arc;

use maybe_async::sync_impl;
use ureq::{Agent, AgentBuilder, Error, Response, Transport};

use super::BaseHttpClient;

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("TLS: {0}")]
    Tls(#[from] native_tls::Error),
    #[error("Transport: {0}")]
    Transport(#[from] Transport),
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Status code: {}", Response::status(.0))]
    StatusCode(Response),
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    agent: Agent,
}

#[sync_impl]
impl BaseHttpClient for HttpClient {
    type Error = HttpClientError;

    #[inline]
    fn get<T>(&self, url: &str) -> Result<T, Self::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request::<T>(url)
    }
}

impl HttpClient {
    pub(crate) fn new() -> Result<Self, HttpClientError> {
        Ok(Self {
            agent: AgentBuilder::new()
                .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
                .build(),
        })
    }

    fn request<T>(&self, url: &str) -> Result<T, HttpClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.agent.get(url).call() {
            Ok(res) => res.into_json::<T>().map_err(Into::into),
            Err(err) => match err {
                Error::Status(_, response) => Err(HttpClientError::StatusCode(response)),
                Error::Transport(transport) => Err(HttpClientError::Transport(transport)),
            },
        }
    }
}
