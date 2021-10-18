//! Item types.

use crate::utility::common_models::{
    APIResource, Description, Effect, GenerationGameIndex, MachineVersionDetail, Name,
    NamedAPIResource, VerboseEffect, VersionGroupFlavorText,
};
use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize, Serialize};

#[pokeapi_struct]
struct Item {
    id: i32,
    name: String,
    cost: i32,
    fling_power: i32,
    fling_effect: NamedAPIResource,
    attributes: Vec<NamedAPIResource>,
    category: NamedAPIResource,
    effect_entries: Vec<VerboseEffect>,
    flavor_text_entries: Vec<VersionGroupFlavorText>,
    game_indices: Vec<GenerationGameIndex>,
    names: Vec<Name>,
    sprites: ItemSprites,
    held_by_pokemon: Vec<ItemHolderPokemon>,
    baby_trigger_for: APIResource,
    machines: Vec<MachineVersionDetail>,
}

#[pokeapi_struct]
struct ItemSprites {
    default: String,
}

#[pokeapi_struct]
struct ItemHolderPokemon {
    pokemon: NamedAPIResource,
    version_details: Vec<ItemHolderPokemonVersionDetail>,
}

#[pokeapi_struct]
struct ItemHolderPokemonVersionDetail {
    rarity: i32,
    version: NamedAPIResource,
}

#[pokeapi_struct]
struct ItemAttribute {
    id: i32,
    name: String,
    items: Vec<NamedAPIResource>,
    names: Vec<Name>,
    descriptions: Vec<Description>,
}

#[pokeapi_struct]
struct ItemCategory {
    id: i32,
    name: String,
    items: Vec<NamedAPIResource>,
    names: Vec<Name>,
    pocket: NamedAPIResource,
}

#[pokeapi_struct]
struct ItemFlingEffect {
    id: i32,
    name: String,
    effect_entries: Vec<Effect>,
    items: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct ItemPockets {
    id: i32,
    name: String,
    categories: Vec<NamedAPIResource>,
    names: Vec<Name>,
}
