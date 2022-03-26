#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "client-reqwest", feature = "client-ureq"))]
compile_error!("Only one of the \"client-reqwest\" and \"client-ureq\" features may be enabled.");

#[cfg(not(any(feature = "client-reqwest", feature = "client-ureq")))]
compile_error!("One of the \"client-reqwest\" or \"client-ureq\" features must be enabled.");

mod http_client;

use http_client::{BaseHttpClient, HttpClient, HttpClientError};
pub use pokeapi_model as model;

/// PokéAPI v2 base URL.
pub const POKEAPI_URL: &str = "https://pokeapi.co/api/v2/";

/// Client error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Client error: {0}")]
    Client(Box<http_client::HttpClientError>),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // TODO: model error?
    // TODO: cache error
}

impl From<HttpClientError> for Error {
    fn from(err: HttpClientError) -> Self {
        Self::Client(Box::new(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
    async fn test_http_client() {
        let client = HttpClient::new().unwrap();
        let endpoint = "machine/1";
        let res = client
            .get::<model::Machine>(&format!("{POKEAPI_URL}{endpoint}"))
            .await;
        println!("{res:#?}");
    }
}
