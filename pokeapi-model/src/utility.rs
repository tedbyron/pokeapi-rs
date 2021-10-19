//! Utility types.

use crate::encounters::{EncounterConditionValue, EncounterMethod};
use crate::games::{Generation, Version, VersionGroup};
use crate::machines::Machine;
use pokeapi_macro::pokeapi_struct;
use std::marker::PhantomData;

/// APIResource type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct APIResource<T> {
    url: String,
    #[serde(skip)]
    _resource_type: PhantomData<*const T>,
}

/// Language type.
///
/// Languages for translations of API resource information.
///
/// [Reference](https://pokeapi.co/docs/v2#languages)
#[pokeapi_struct]
struct Language {
    id: i32,
    name: String,
    official: bool,
    iso639: String,
    iso3166: String,
    names: Vec<Name>,
}

/// Description type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Description {
    description: String,
    language: NamedAPIResource<Language>,
}

/// Effect type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Effect {
    effect: String,
    language: NamedAPIResource<Language>,
}

/// Encounter type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Encounter {
    min_level: i32,
    max_level: i32,
    condition_values: Vec<NamedAPIResource<EncounterConditionValue>>,
    chance: i32,
    method: NamedAPIResource<EncounterMethod>,
}

/// FlavorText type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct FlavorText {
    flavor_text: String,
    language: NamedAPIResource<Language>,
    version: NamedAPIResource<Version>,
}

/// GenerationGameIndex type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct GenerationGameIndex {
    game_index: i32,
    generation: NamedAPIResource<Generation>,
}

/// MachineVersionDetail type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct MachineVersionDetail {
    machine: APIResource<Machine>,
    version_group: NamedAPIResource<VersionGroup>,
}

/// Name type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Name {
    name: String,
    language: NamedAPIResource<Language>,
}

/// NamedAPIResource type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct NamedAPIResource<T> {
    description: String,
    url: String,
    #[serde(skip)]
    _resource_type: PhantomData<*const T>,
}

/// VerboseEffect type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VerboseEffect {
    effect: String,
    short_effect: String,
    language: NamedAPIResource<Language>,
}

/// VersionEncounterDetail type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionEncounterDetail {
    version: NamedAPIResource<Version>,
    max_chance: i32,
    encounter_details: Vec<Encounter>,
}

/// VersionGameIndex type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionGameIndex {
    game_index: i32,
    version: NamedAPIResource<Version>,
}

/// VersionGroupFlavorText type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionGroupFlavorText {
    text: String,
    language: NamedAPIResource<Language>,
    version_group: NamedAPIResource<VersionGroup>,
}
