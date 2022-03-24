#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod berries;
mod contests;
mod encounters;
mod evolution;
mod games;
mod items;
mod locations;
mod machines;
mod moves;
mod pokemon;
mod resource_lists_pagination;
mod utility;

pub use {
    berries::*, contests::*, encounters::*, evolution::*, games::*, items::*, locations::*,
    machines::*, moves::*, pokemon::*, resource_lists_pagination::*, utility::*,
};
