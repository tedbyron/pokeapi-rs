//! Location types.

use crate::utility::common_models::{
    GenerationGameIndex, Name, NamedAPIResource, VersionEncounterDetail,
};
use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize};

#[pokeapi_struct]
struct Location {
    id: i32,
    name: String,
    region: NamedAPIResource,
    names: Vec<Name>,
    game_indices: Vec<GenerationGameIndex>,
    areas: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct LocationArea {
    id: i32,
    name: String,
    game_index: i32,
    encounter_method_rates: Vec<EncounterMethodRate>,
    location: NamedAPIResource,
    names: Name,
    pokemon_encounters: Vec<PokemonEncounter>,
}

#[pokeapi_struct]
struct EncounterMethodRate {
    encounter_method: NamedAPIResource,
    version_details: Vec<EncounterVersionDetails>,
}

#[pokeapi_struct]
struct EncounterVersionDetails {
    rate: i32,
    version: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonEncounter {
    pokemon: NamedAPIResource,
    version_details: Vec<VersionEncounterDetail>,
}

#[pokeapi_struct]
struct PalParkArea {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_encounters: Vec<PalParkEncounterSpecies>,
}

#[pokeapi_struct]
struct PalParkEncounterSpecies {
    base_score: i32,
    rate: i32,
    pokemon_species: NamedAPIResource,
}

#[pokeapi_struct]
struct Region {
    id: i32,
    locations: Vec<NamedAPIResource>,
    name: String,
    names: Vec<Name>,
    main_generation: NamedAPIResource,
    pokedexes: Vec<NamedAPIResource>,
    version_groups: Vec<NamedAPIResource>,
}
