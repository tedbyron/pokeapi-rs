use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct APIResource {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Description {
    pub description: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Effect {
    pub effect: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Encounter {
    pub min_level: i32,
    pub max_level: i32,
    pub condition_values: Vec<NamedAPIResource>,
    pub chance: i32,
    pub method: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerationGameIndex {
    pub game_index: i32,
    pub generation: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MachineVersionDetail {
    pub machine: APIResource,
    pub version_group: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamedAPIResource {
    pub description: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerboseEffect {
    pub effect: String,
    pub short_effect: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionEncounterDetail {
    pub version: NamedAPIResource,
    pub max_chance: i32,
    pub encounter_details: Vec<Encounter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionGameIndex {
    pub game_index: i32,
    pub version: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionGroupFlavorText {
    pub text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
}
