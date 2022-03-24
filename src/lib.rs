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

mod client;
mod error;

// pub use client::*;
pub use error::Error;

/// PokéAPI v2 base URL.
pub const POKEAPI_URL: &str = "https://pokeapi.co/api/v2/";
