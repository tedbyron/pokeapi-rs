//! Location types.

use crate::encounters::EncounterMethod;
use crate::games::{Generation, Pokedex, Version, VersionGroup};
use crate::pokemon::{Pokemon, PokemonSpecies};
use crate::utility::{GenerationGameIndex, Name, NamedAPIResource, VersionEncounterDetail};
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct Location {
    id: i32,
    name: String,
    region: NamedAPIResource<Region>,
    names: Vec<Name>,
    game_indices: Vec<GenerationGameIndex>,
    areas: Vec<NamedAPIResource<LocationArea>>,
}

#[pokeapi_struct]
struct LocationArea {
    id: i32,
    name: String,
    game_index: i32,
    encounter_method_rates: Vec<EncounterMethodRate>,
    location: NamedAPIResource<Location>,
    names: Name,
    pokemon_encounters: Vec<PokemonEncounter>,
}

#[pokeapi_struct]
struct EncounterMethodRate {
    encounter_method: NamedAPIResource<EncounterMethod>,
    version_details: Vec<EncounterVersionDetails>,
}

#[pokeapi_struct]
struct EncounterVersionDetails {
    rate: i32,
    version: NamedAPIResource<Version>,
}

#[pokeapi_struct]
struct PokemonEncounter {
    pokemon: NamedAPIResource<Pokemon>,
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
    pokemon_species: NamedAPIResource<PokemonSpecies>,
}

#[pokeapi_struct]
struct Region {
    id: i32,
    locations: Vec<NamedAPIResource<Location>>,
    name: String,
    names: Vec<Name>,
    main_generation: NamedAPIResource<Generation>,
    pokedexes: Vec<NamedAPIResource<Pokedex>>,
    version_groups: Vec<NamedAPIResource<VersionGroup>>,
}
