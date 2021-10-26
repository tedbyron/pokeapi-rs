use super::{error::Error, Message};
use iced::{futures, image, Alignment, Column, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub struct Pokemon {
    number: u16,
    pub name: String,
    description: String,
    image: image::Handle,
    image_viewer: image::viewer::State,
}

impl Pokemon {
    const MAX_ID: u16 = 898;

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(image::Viewer::new(
                &mut self.image_viewer,
                self.image.clone(),
            ))
            .push(
                Column::new()
                    .spacing(20)
                    .push(
                        Row::new()
                            .align_items(Alignment::Center)
                            .spacing(20)
                            .push(Text::new(&self.name).size(30).width(Length::Fill))
                            .push(
                                Text::new(format!("#{}", self.number))
                                    .size(20)
                                    .color([0.5, 0.5, 0.5]),
                            ),
                    )
                    .push(Text::new(&self.description)),
            )
            .into()
    }

    pub async fn search() -> Result<Self, Error> {
        use rand::Rng;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct Entry {
            id: u32,
            name: String,
            flavor_text_entries: Vec<FlavorText>,
        }

        #[derive(Debug, Deserialize)]
        struct FlavorText {
            flavor_text: String,
            language: Language,
        }

        #[derive(Debug, Deserialize)]
        struct Language {
            name: String,
        }

        let id = {
            let mut rng = rand::rngs::OsRng::default();
            rng.gen_range(1..=Self::MAX_ID)
        };

        let fetch_entry = async {
            let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
            reqwest::get(&url).await?.json().await
        };

        let (entry, image): (Entry, _) =
            futures::future::try_join(fetch_entry, Self::fetch_image(id)).await?;

        let description = entry
            .flavor_text_entries
            .iter()
            .find(|text| text.language.name == "en")
            .ok_or(Error::LanguageError)?;

        Ok(Self {
            number: id,
            name: entry.name.to_uppercase(),
            description: description
                .flavor_text
                .chars()
                .map(|c| if c.is_control() { ' ' } else { c })
                .collect(),
            image,
            image_viewer: image::viewer::State::new(),
        })
    }

    async fn fetch_image(id: u16) -> Result<image::Handle, reqwest::Error> {
        let url = format!(
            "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
            id
        );

        if cfg!(target_arch = "wasm32") {
            Ok(image::Handle::from_path(url))
        } else {
            let bytes = reqwest::get(&url).await?.bytes().await?;
            Ok(image::Handle::from_memory(bytes.as_ref().to_vec()))
        }
    }
}
