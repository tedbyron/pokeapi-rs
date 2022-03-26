use maybe_async::async_impl;
use reqwest::{Client, Error, Response};

use super::BaseHttpClient;

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("Reqwest: {0}")]
    Reqwest(#[from] Error),
    #[error("Status code: {}", Response::status(.0))]
    StatusCode(Response),
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
}

#[async_impl]
impl BaseHttpClient for HttpClient {
    type Error = HttpClientError;

    #[inline]
    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request::<T>(url).await
    }
}

impl HttpClient {
    pub(crate) fn new() -> Result<Self, HttpClientError> {
        Ok(Self {
            client: Client::builder()
                .tls_built_in_root_certs(true)
                .use_native_tls()
                .build()?,
        })
    }

    async fn request<T>(&self, url: &str) -> Result<T, HttpClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let res = self.client.get(url).send().await?;

        if res.status().is_success() {
            res.json::<T>().await.map_err(Into::into)
        } else {
            Err(HttpClientError::StatusCode(res))
        }
    }
}
