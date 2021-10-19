//! Resource list/pagination types.

use crate::utility::NamedAPIResource;
use pokeapi_macro::pokeapi_struct;

/// NamedAPIResourceList type.
///
/// Calling any API endpoint without a resource ID or name will return a
/// paginated list of available resources for that API. By default, a list
/// "page" will contain up to 20 resources. If you would like to change this
/// just add a 'limit' query parameter to the GET request, e.g. `?=60`. You can
/// use 'offset' to move to the next page, e.g. `?limit=60&offset=60`.
///
/// [Reference](https://pokeapi.co/docs/v2#namedapiresourcelist)
#[pokeapi_struct]
struct NamedAPIResourceList<T> {
    count: i32,
    next: String,
    previous: bool,
    results: Vec<NamedAPIResource<T>>,
}
