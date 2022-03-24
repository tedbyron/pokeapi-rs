//! Machine types.

use pokeapi_macro::pokeapi_struct;

use crate::{Item, Move, NamedAPIResource, VersionGroup};

#[pokeapi_struct]
struct Machine {
    id: i32,
    item: NamedAPIResource<Item>,
    #[serde(rename = "move")]
    move_: NamedAPIResource<Move>,
    version_group: NamedAPIResource<VersionGroup>,
}
