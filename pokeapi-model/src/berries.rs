//! Berry types.

use pokeapi_macro::pokeapi_struct;

use crate::{ContestType, Item, Name, NamedAPIResource, Type};

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

#[pokeapi_struct]
struct BerryFlavorMap {
    potency: i32,
    flavor: NamedAPIResource<BerryFlavor>,
}

#[pokeapi_struct]
struct BerryFirmness {
    id: i32,
    name: String,
    berries: Vec<NamedAPIResource<Berry>>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct BerryFlavor {
    id: i32,
    name: String,
    berries: Vec<FlavorBerryMap>,
    contest_type: NamedAPIResource<ContestType>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct FlavorBerryMap {
    potency: i32,
    berry: NamedAPIResource<Berry>,
}
