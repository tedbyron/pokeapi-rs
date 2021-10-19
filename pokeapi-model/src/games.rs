//! Game types.

use crate::locations::Region;
use crate::moves::{Move, MoveLearnMethod};
use crate::pokemon::{Ability, PokemonSpecies, Type};
use crate::utility::{Description, Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct Generation {
    id: i32,
    name: String,
    abilities: Vec<NamedAPIResource<Ability>>,
    names: Vec<Name>,
    main_region: NamedAPIResource<Region>,
    moves: Vec<NamedAPIResource<Move>>,
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
    types: Vec<NamedAPIResource<Type>>,
    version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

#[pokeapi_struct]
struct Pokedex {
    id: i32,
    name: String,
    is_main_series: bool,
    descriptions: Vec<Description>,
    names: Vec<Name>,
    pokemon_entries: Vec<PokemonEntry>,
    region: NamedAPIResource<Region>,
    version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

#[pokeapi_struct]
struct PokemonEntry {
    entry_number: i32,
    pokemon_species: NamedAPIResource<PokemonSpecies>,
}

#[pokeapi_struct]
struct Version {
    id: i32,
    name: String,
    names: Vec<Name>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct VersionGroup {
    id: i32,
    name: String,
    order: i32,
    generation: NamedAPIResource<Generation>,
    move_learn_methods: Vec<NamedAPIResource<MoveLearnMethod>>,
    pokedexes: Vec<NamedAPIResource<Pokedex>>,
    regions: Vec<NamedAPIResource<Region>>,
    versions: Vec<NamedAPIResource<Version>>,
}
