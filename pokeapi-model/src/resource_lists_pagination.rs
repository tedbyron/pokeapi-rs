//! Resource list/pagination types.

use crate::utility::NamedAPIResource;
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct NamedAPIResourceList<T> {
    count: i32,
    next: String,
    previous: bool,
    results: Vec<NamedAPIResource<T>>,
}
