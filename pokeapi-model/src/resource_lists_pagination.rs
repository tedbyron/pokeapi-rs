//! Resource list/pagination types.

use pokeapi_macro::pokeapi_struct;

use crate::NamedAPIResource;

#[pokeapi_struct]
struct NamedAPIResourceList<T> {
    count: i32,
    next: String,
    previous: bool,
    results: Vec<NamedAPIResource<T>>,
}
