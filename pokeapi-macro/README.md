<div align="center">
  <h1><code>pokeapi-macro</code></h1>

  <p>
    <strong>Attribute macro for <a href="../pokeapi-model"><code>pokeapi-model</code></a> structs.</strong>
  </p>
</div>

## Usage

All API bindings in `pokeapi-model` must be `Debug`, `Clone`, `PartialEq`, and `Deserialize`, as well as have `pub` visibility.

Consider the following example:

```rs
use pokeapi-macro::pokeapi_struct;

#[pokeapi_struct]
struct NamedAPIResource {
  description: String,
  url: String,
}
```

This attribute will output the `struct` with required derived traits and visibility:

```rs
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NamedAPIResource {
  pub description: String,
  pub url: String,
}
```
