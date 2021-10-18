//! Encounter types.

use crate::utility::common_models::{Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize};

/// EncounterMethod type.
///
/// Methods by which the player might can encounter Pokémon in the wild, e.g.,
/// walking in tall grass.
///
/// [Reference](https://pokeapi.co/docs/v2#encounter-methods)
#[pokeapi_struct]
struct EncounterMethod {
    id: i32,
    name: String,
    order: i32,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct EncounterCondition {
    id: i32,
    name: String,
    names: Vec<Name>,
    values: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct EncounterConditionValues {
    id: i32,
    name: String,
    condition: NamedAPIResource,
    names: Vec<Name>,
}
