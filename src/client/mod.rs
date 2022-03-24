#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

use maybe_async::maybe_async;

#[cfg(feature = "client-reqwest")]
pub use self::reqwest::HttpError;
#[cfg(feature = "client-ureq")]
pub use self::ureq::HttpError;

#[maybe_async]
pub trait HttpClient {
    type Error;

    async fn get<T>(&self, endpoint_suffix: &str) -> Result<T, Self::Error>
    where
        T: serde::de::DeserializeOwned;
}
