//! Evolution types.

use pokeapi_macro::pokeapi_struct;

use crate::{Item, Location, Move, Name, NamedAPIResource, PokemonSpecies, Type};

#[pokeapi_struct]
struct EvolutionChain {
    id: i32,
    baby_trigger_item: NamedAPIResource<Item>,
    chain: ChainLink,
}

#[pokeapi_struct]
struct ChainLink {
    is_baby: bool,
    species: NamedAPIResource<PokemonSpecies>,
    evolution_details: Vec<EvolutionDetail>,
    evolves_to: Vec<Self>,
}

#[pokeapi_struct]
struct EvolutionDetail {
    item: NamedAPIResource<Item>,
    trigger: NamedAPIResource<EvolutionTrigger>,
    gender: i32,
    held_item: NamedAPIResource<Item>,
    known_move: NamedAPIResource<Move>,
    known_move_type: NamedAPIResource<Type>,
    location: NamedAPIResource<Location>,
    min_level: i32,
    min_happiness: i32,
    min_beauty: i32,
    min_affection: i32,
    needs_overworld_rain: bool,
    party_species: NamedAPIResource<PokemonSpecies>,
    party_type: NamedAPIResource<Type>,
    relative_physical_stats: i32,
    time_of_day: String,
    trade_species: NamedAPIResource<PokemonSpecies>,
    turn_upside_down: bool,
}

#[pokeapi_struct]
struct EvolutionTrigger {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}
