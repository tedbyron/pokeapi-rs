use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct APIResource<'a> {
    url: &'a str,
}

#[derive(Deserialize)]
pub(super) struct Description<'a> {
    description: &'a str,
    language: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct Effect<'a> {
    effect: &'a str,
    language: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct Encounter<'a> {
    min_level: i32,
    max_level: i32,
    condition_values: Vec<NamedAPIResource<'a>>,
    chance: i32,
    method: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct FlavorText<'a> {
    flavor_text: &'a str,
    language: NamedAPIResource<'a>,
    version: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct GenerationGameIndex<'a> {
    game_index: i32,
    generation: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct MachineVersionDetail<'a> {
    machine: APIResource<'a>,
    version_group: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct Name<'a> {
    name: &'a str,
    language: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct NamedAPIResource<'a> {
    description: &'a str,
    url: &'a str,
}

#[derive(Deserialize)]
pub(super) struct VerboseEffect<'a> {
    effect: &'a str,
    short_effect: &'a str,
    language: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct VersionEncounterDetail<'a> {
    version: NamedAPIResource<'a>,
    max_chance: i32,
    encounter_details: Vec<Encounter<'a>>,
}

#[derive(Deserialize)]
pub(super) struct VersionGameIndex<'a> {
    game_index: i32,
    version: NamedAPIResource<'a>,
}

#[derive(Deserialize)]
pub(super) struct VersionGroupFlavorText<'a> {
    text: &'a str,
    language: NamedAPIResource<'a>,
    version_group: NamedAPIResource<'a>,
}
