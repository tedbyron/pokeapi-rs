//! Game types.

use crate::utility::common_models::{Description, Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;
use serde::Deserialize;

#[pokeapi_struct]
struct Generation {
    id: i32,
    name: String,
    abilities: Vec<NamedAPIResource>,
    names: Vec<Name>,
    main_region: NamedAPIResource,
    moves: Vec<NamedAPIResource>,
    pokemon_species: Vec<NamedAPIResource>,
    types: Vec<NamedAPIResource>,
    version_groups: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct Pokedex {
    id: i32,
    name: String,
    is_main_series: bool,
    descriptions: Vec<Description>,
    names: Vec<Name>,
    pokemon_entries: Vec<PokemonEntry>,
    region: NamedAPIResource,
    version_groups: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct PokemonEntry {
    entry_number: i32,
    pokemon_species: NamedAPIResource,
}

#[pokeapi_struct]
struct Version {
    id: i32,
    name: String,
    names: Vec<Name>,
    version_group: NamedAPIResource,
}

#[pokeapi_struct]
struct VersionGroup {
    id: i32,
    name: String,
    order: i32,
    generation: NamedAPIResource,
    move_learn_methods: Vec<NamedAPIResource>,
    pokedexes: Vec<NamedAPIResource>,
    regions: Vec<NamedAPIResource>,
    versions: Vec<NamedAPIResource>,
}
