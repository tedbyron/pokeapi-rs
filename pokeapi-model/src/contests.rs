//! Contest types.

use crate::berries::BerryFlavor;
use crate::moves::Move;
use crate::utility::{Effect, FlavorText, Language, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;

/// ContestType type.
///
/// Contest types are categories judges used to weigh a Pokémon's condition in
/// Pokémon contests.
///
/// [Reference](https://pokeapi.co/docs/v2#contest-types)
#[pokeapi_struct]
struct ContestType {
    id: i32,
    name: String,
    berry_flavor: NamedAPIResource<BerryFlavor>,
    names: Vec<ContestName>,
}

/// ContestName type.
///
/// [Reference](https://pokeapi.co/docs/v2#contest-types)
#[pokeapi_struct]
struct ContestName {
    name: String,
    color: String,
    language: NamedAPIResource<Language>,
}

/// ContestEffect type.
///
/// Contest effects refer to the effects of moves when used in contests.
///
/// [Reference](https://pokeapi.co/docs/v2#contest-effects)
#[pokeapi_struct]
struct ContestEffect {
    id: i32,
    appeal: i32,
    jam: i32,
    effect_entries: Vec<Effect>,
    flavor_text_entries: Vec<FlavorText>,
}

/// SuperContestEffect type.
///
/// Super contest effects refer to the effects of moves when used in super contests.
///
/// [Reference](https://pokeapi.co/docs/v2#super-contest-effects)
#[pokeapi_struct]
struct SuperContestEffect {
    id: i32,
    appeal: i32,
    flavor_text_entries: Vec<FlavorText>,
    moves: Vec<NamedAPIResource<Move>>,
}
