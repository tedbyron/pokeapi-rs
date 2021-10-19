//! Item types.

use crate::evolution::EvolutionChain;
use crate::games::Version;
use crate::pokemon::Pokemon;
use crate::utility::{
    APIResource, Description, Effect, GenerationGameIndex, MachineVersionDetail, Name,
    NamedAPIResource, VerboseEffect, VersionGroupFlavorText,
};
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct Item {
    id: i32,
    name: String,
    cost: i32,
    fling_power: i32,
    fling_effect: NamedAPIResource<ItemFlingEffect>,
    attributes: Vec<NamedAPIResource<ItemAttribute>>,
    category: NamedAPIResource<ItemCategory>,
    effect_entries: Vec<VerboseEffect>,
    flavor_text_entries: Vec<VersionGroupFlavorText>,
    game_indices: Vec<GenerationGameIndex>,
    names: Vec<Name>,
    sprites: ItemSprites,
    held_by_pokemon: Vec<ItemHolderPokemon>,
    baby_trigger_for: APIResource<EvolutionChain>,
    machines: Vec<MachineVersionDetail>,
}

#[pokeapi_struct]
struct ItemSprites {
    default: String,
}

#[pokeapi_struct]
struct ItemHolderPokemon {
    pokemon: NamedAPIResource<Pokemon>,
    version_details: Vec<ItemHolderPokemonVersionDetail>,
}

#[pokeapi_struct]
struct ItemHolderPokemonVersionDetail {
    rarity: i32,
    version: NamedAPIResource<Version>,
}

#[pokeapi_struct]
struct ItemAttribute {
    id: i32,
    name: String,
    items: Vec<NamedAPIResource<Item>>,
    names: Vec<Name>,
    descriptions: Vec<Description>,
}

#[pokeapi_struct]
struct ItemCategory {
    id: i32,
    name: String,
    items: Vec<NamedAPIResource<Item>>,
    names: Vec<Name>,
    pocket: NamedAPIResource<ItemPocket>,
}

#[pokeapi_struct]
struct ItemFlingEffect {
    id: i32,
    name: String,
    effect_entries: Vec<Effect>,
    items: Vec<NamedAPIResource<Item>>,
}

#[pokeapi_struct]
struct ItemPocket {
    id: i32,
    name: String,
    categories: Vec<NamedAPIResource<ItemCategory>>,
    names: Vec<Name>,
}
