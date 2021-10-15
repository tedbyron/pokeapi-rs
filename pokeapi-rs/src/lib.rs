#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

pub mod client;
pub mod structs;

pub use ureq::json;
