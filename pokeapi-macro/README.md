<div align="center">
  <h1><code>pokeapi-macro</code></h1>

  <p>
    <strong>Attribute macro for <code>pokeapi-model</code>.</strong>
  </p>
</div>

## Usage

Consider the following example:

```rust
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct NamedAPIResource<T> {
    description: String,
    url: String,
    _resource_type: std::marker::PhantomData<*const T>,
}
```

This attribute will output the `struct` with required derived traits and visibility:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct NamedAPIResource<T> {
    pub description: String,
    pub url: String,
    #[serde(skip)]
    _resource_type: std::marker::PhantomData<*const T>
}
```
