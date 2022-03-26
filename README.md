<div align="center">
  <h1><code>pokeapi-rs</code></h1>

  <p>
    <strong>Rust bindings/wrapper and client for PokéAPI v2.</strong>
  </p>
</div>

Rust bindings/wrapper and client for PokéAPI v2. WIP with future support for request/response/image
caching and sync/async clients.

```toml
# Cargo.toml
[dependencies]
pokeapi = "1"
```

# Usage

```rs
use pokeapi::{Client};
use pokeapi::model::Pokemon;

let client = Client::new();

// Get a resource by name:
client.pokemon('pikachu')
// Or by ID:
client.pokemon('25')
```

TODO
- doc comment generation on model types from module name
- add endpoints to model through macro
- native tls - ureq needs `Agent` in `Client`
- cache
