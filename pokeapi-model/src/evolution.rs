//! Evolution types.

use crate::utility::common_models::{Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize};

#[pokeapi_struct]
struct EvolutionChain {
    id: i32,
    baby_trigger_item: NamedAPIResource,
    chain: ChainLink,
}

#[pokeapi_struct]
struct ChainLink {
    is_baby: bool,
    species: NamedAPIResource,
    evolution_details: Vec<EvolutionDetail>,
    evolves_to: Vec<Self>,
}

#[pokeapi_struct]
struct EvolutionDetail {
    item: NamedAPIResource,
    trigger: NamedAPIResource,
    gender: i32,
    held_item: NamedAPIResource,
    known_move: NamedAPIResource,
    known_move_type: NamedAPIResource,
    location: NamedAPIResource,
    min_level: i32,
    min_happiness: i32,
    min_beauty: i32,
    min_affection: i32,
    needs_overworld_rain: bool,
    party_species: NamedAPIResource,
    party_type: NamedAPIResource,
    relative_physical_stats: i32,
    time_of_day: String,
    trade_species: NamedAPIResource,
    turn_upside_down: bool,
}

#[pokeapi_struct]
struct EvolutionTrigger {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource>,
}
