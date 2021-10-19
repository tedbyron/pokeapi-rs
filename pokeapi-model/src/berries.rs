//! Berry types.

use crate::contests::ContestType;
use crate::items::Item;
use crate::pokemon::Type;
use crate::utility::{Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;

/// Berry type.
///
/// Berries are small fruits that can provide HP and status condition
/// restoration, stat enhancement, and even damage negation when eaten by
/// Pokémon.
///
/// [Reference](https://pokeapi.co/docs/v2#berries)
#[pokeapi_struct]
struct Berry {
    id: i32,
    name: String,
    growth_time: i32,
    max_harvest: i32,
    natural_gift_power: i32,
    size: i32,
    smoothness: i32,
    soil_dryness: i32,
    firmness: NamedAPIResource<BerryFirmness>,
    flavors: Vec<BerryFlavorMap>,
    item: NamedAPIResource<Item>,
    natural_gift_type: NamedAPIResource<Type>,
}

/// Berry flavor map type.
///
/// [Reference](https://pokeapi.co/docs/v2#berries)
#[pokeapi_struct]
struct BerryFlavorMap {
    potency: i32,
    flavor: NamedAPIResource<BerryFlavor>,
}

/// Berry firmness type.
///
/// Berries can be soft or hard.
///
/// [Reference](https://pokeapi.co/docs/v2#berry-firmnesses)
#[pokeapi_struct]
struct BerryFirmness {
    id: i32,
    name: String,
    berries: Vec<NamedAPIResource<Berry>>,
    names: Vec<Name>,
}

/// Berry flavor type.
///
/// Flavors determine whether a Pokémon will benefit or suffer from eating a
/// berry based on their nature.
///
/// [Reference](https://pokeapi.co/docs/v2#berry-flavors)
#[pokeapi_struct]
struct BerryFlavor {
    id: i32,
    name: String,
    berries: Vec<FlavorBerryMap>,
    contest_type: NamedAPIResource<ContestType>,
    names: Vec<Name>,
}

/// Flavor berry map type.
///
/// [Reference](https://pokeapi.co/docs/v2#berry-flavors)
#[pokeapi_struct]
struct FlavorBerryMap {
    potency: i32,
    berry: NamedAPIResource<Berry>,
}
