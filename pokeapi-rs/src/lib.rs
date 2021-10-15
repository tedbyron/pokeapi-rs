#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

mod client;
mod structs;

pub use client::*;
pub use structs::*;
