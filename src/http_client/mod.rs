#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

use maybe_async::maybe_async;

#[cfg(feature = "client-reqwest")]
pub use self::reqwest::{HttpClient, HttpClientError};
#[cfg(feature = "client-ureq")]
pub use self::ureq::{HttpClient, HttpClientError};

#[maybe_async]
pub trait BaseHttpClient {
    type Error;

    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
    where
        T: serde::de::DeserializeOwned;
}
