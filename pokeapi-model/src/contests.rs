//! Contest types.

use pokeapi_macro::pokeapi_struct;

use crate::{BerryFlavor, Effect, FlavorText, Language, Move, NamedAPIResource};

#[pokeapi_struct]
struct ContestType {
    id: i32,
    name: String,
    berry_flavor: NamedAPIResource<BerryFlavor>,
    names: Vec<ContestName>,
}

#[pokeapi_struct]
struct ContestName {
    name: String,
    color: String,
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct ContestEffect {
    id: i32,
    appeal: i32,
    jam: i32,
    effect_entries: Vec<Effect>,
    flavor_text_entries: Vec<FlavorText>,
}

#[pokeapi_struct]
struct SuperContestEffect {
    id: i32,
    appeal: i32,
    flavor_text_entries: Vec<FlavorText>,
    moves: Vec<NamedAPIResource<Move>>,
}
