mod error;
mod pokemon;
mod style;

use error::Error;
use iced::{
    button, Alignment, Application, Button, Column, Command, Container, Element, Length, Text,
};
use pokemon::Pokemon;

#[derive(Debug)]
pub enum Pokedex {
    Loading,
    Loaded {
        pokemon: Pokemon,
        search: button::State,
    },
    Errored {
        error: Error,
        try_again: button::State,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    PokémonFound(Result<Pokemon, Error>),
    Search,
}

impl Application for Pokedex {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(Pokemon::search(), Message::PokémonFound),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            Self::Loading => "Loading",
            Self::Loaded { pokemon, .. } => &pokemon.name,
            Self::Errored { .. } => "Whoops!",
        };

        format!("Pok\u{e9}dex - {}", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PokémonFound(Ok(pokemon)) => {
                *self = Self::Loaded {
                    pokemon,
                    search: button::State::new(),
                };

                Command::none()
            }
            Message::PokémonFound(Err(error)) => {
                *self = Self::Errored {
                    error,
                    try_again: button::State::new(),
                };

                Command::none()
            }
            Message::Search => {
                if let Self::Loading = self {
                    Command::none()
                } else {
                    *self = Self::Loading;

                    Command::perform(Pokemon::search(), Message::PokémonFound)
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self {
            Self::Loading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Searching for Pok\u{e9}mon...").size(40)),
            Self::Loaded { pokemon, search } => Column::new()
                .max_width(500)
                .spacing(20)
                .align_items(Alignment::End)
                .push(pokemon.view())
                .push(button(search, "Keep searching!").on_press(Message::Search)),
            Self::Errored { try_again, .. } => Column::new()
                .spacing(20)
                .align_items(Alignment::End)
                .push(Text::new("Whoops! Something went wrong...").size(40))
                .push(button(try_again, "Try again").on_press(Message::Search)),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::Primary)
}
