#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod app;

use iced::{Application, Settings};

#[allow(clippy::missing_errors_doc)]
pub fn main() -> iced::Result {
    app::Pokedex::run(Settings::default())
}
