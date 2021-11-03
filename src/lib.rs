#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod client;

pub use client::*;
