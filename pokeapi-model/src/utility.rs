//! Utility types.

use crate::encounters::{EncounterConditionValue, EncounterMethod};
use crate::games::{Generation, Version, VersionGroup};
use crate::machines::Machine;
use pokeapi_macro::pokeapi_struct;
use std::marker::PhantomData;

#[pokeapi_struct]
struct APIResource<T> {
    url: String,
    #[serde(skip)]
    _resource_type: PhantomData<*const T>,
}

#[pokeapi_struct]
struct Language {
    id: i32,
    name: String,
    official: bool,
    iso639: String,
    iso3166: String,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct Description {
    description: String,
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct Effect {
    effect: String,
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct Encounter {
    min_level: i32,
    max_level: i32,
    condition_values: Vec<NamedAPIResource<EncounterConditionValue>>,
    chance: i32,
    method: NamedAPIResource<EncounterMethod>,
}

#[pokeapi_struct]
struct FlavorText {
    flavor_text: String,
    language: NamedAPIResource<Language>,
    version: NamedAPIResource<Version>,
}

#[pokeapi_struct]
struct GenerationGameIndex {
    game_index: i32,
    generation: NamedAPIResource<Generation>,
}

#[pokeapi_struct]
struct MachineVersionDetail {
    machine: APIResource<Machine>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct Name {
    name: String,
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct NamedAPIResource<T> {
    description: String,
    url: String,
    #[serde(skip)]
    _resource_type: PhantomData<*const T>,
}

#[pokeapi_struct]
struct VerboseEffect {
    effect: String,
    short_effect: String,
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct VersionEncounterDetail {
    version: NamedAPIResource<Version>,
    max_chance: i32,
    encounter_details: Vec<Encounter>,
}

#[pokeapi_struct]
struct VersionGameIndex {
    game_index: i32,
    version: NamedAPIResource<Version>,
}

#[pokeapi_struct]
struct VersionGroupFlavorText {
    text: String,
    language: NamedAPIResource<Language>,
    version_group: NamedAPIResource<VersionGroup>,
}
