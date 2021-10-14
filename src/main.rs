#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use iced::{Application, Settings};

#[allow(clippy::missing_errors_doc)]
pub fn main() -> iced::Result {
    app::Pokedex::run(Settings::default())
}
