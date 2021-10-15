<div align="center">
  <h1><code>pokeapi-macro</code></h1>

  <p>
    <strong>Attribute macro for <a href="../pokeapi-rs"><code>pokeapi-rs</code></a> structs.</strong>
  </p>
</div>

## Usage

All API bindings in `pokeapi-rs` must be `Debug`, `Clone`, and `Deserialize`, as well as have `pub` visibility.

Consider the following attribute example:

``` rust
use pokeapi-macro::pub_struct;

#[pub_struct]
struct NamedAPIResource {
  description: String,
  url: String,
}
```

This attribute will output the `struct` with required derived traits and visibility:

``` rust
#[derive(Debug, Clone, Deserialize)]
pub struct NamedAPIResource {
  pub description: String,
  pub url: String,
}
```
