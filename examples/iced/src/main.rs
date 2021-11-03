#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![doc = include_str!("../README.md")]

mod app;

use iced::{Application, Settings};

#[allow(clippy::missing_errors_doc)]
pub fn main() -> iced::Result {
    app::Pokedex::run(Settings::default())
}
