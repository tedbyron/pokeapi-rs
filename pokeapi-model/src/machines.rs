//! Machine types.

use crate::games::VersionGroup;
use crate::items::Item;
use crate::moves::Move;
use crate::utility::NamedAPIResource;
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct Machine {
    id: i32,
    item: NamedAPIResource<Item>,
    #[serde(rename = "move")]
    move_: NamedAPIResource<Move>,
    version_group: NamedAPIResource<VersionGroup>,
}
