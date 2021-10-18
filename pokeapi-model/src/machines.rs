//! Machine types.

use crate::utility::common_models::NamedAPIResource;
use pokeapi_macro::pokeapi_struct;
use serde::Deserialize;

#[pokeapi_struct]
struct Machine {
    id: i32,
    item: NamedAPIResource,
    #[serde(rename = "move")]
    move_: NamedAPIResource,
    version_group: NamedAPIResource,
}
