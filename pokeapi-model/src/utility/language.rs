//! Languages for translations of API resource information.

use crate::utility::common_models::Name;
use pokeapi_macro::pokeapi_struct;
use serde::{Deserialize, Serialize};

/// Language type.
///
/// Languages for translations of API resource information.
///
/// [Reference](https://pokeapi.co/docs/v2#languages)
#[pokeapi_struct]
struct Language {
    id: i32,
    name: String,
    official: bool,
    iso639: String,
    iso3166: String,
    names: Vec<Name>,
}
