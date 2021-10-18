//! Common models.

use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize, Serialize};

/// APIResource type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct APIResource {
    url: String,
}

/// Description type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Description {
    description: String,
    language: NamedAPIResource,
}

/// Effect type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Effect {
    effect: String,
    language: NamedAPIResource,
}

/// Encounter type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Encounter {
    min_level: i32,
    max_level: i32,
    condition_values: Vec<NamedAPIResource>,
    chance: i32,
    method: NamedAPIResource,
}

/// FlavorText type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct FlavorText {
    flavor_text: String,
    language: NamedAPIResource,
    version: NamedAPIResource,
}

/// GenerationGameIndex type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct GenerationGameIndex {
    game_index: i32,
    generation: NamedAPIResource,
}

/// MachineVersionDetail type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct MachineVersionDetail {
    machine: APIResource,
    version_group: NamedAPIResource,
}

/// Name type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct Name {
    name: String,
    language: NamedAPIResource,
}

/// NamedAPIResource type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct NamedAPIResource {
    description: String,
    url: String,
}

/// VerboseEffect type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VerboseEffect {
    effect: String,
    short_effect: String,
    language: NamedAPIResource,
}

/// VersionEncounterDetail type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionEncounterDetail {
    version: NamedAPIResource,
    max_chance: i32,
    encounter_details: Vec<Encounter>,
}

/// VersionGameIndex type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionGameIndex {
    game_index: i32,
    version: NamedAPIResource,
}

/// VersionGroupFlavorText type.
///
/// [Reference](https://pokeapi.co/docs/v2#common-models)
#[pokeapi_struct]
struct VersionGroupFlavorText {
    text: String,
    language: NamedAPIResource,
    version_group: NamedAPIResource,
}
