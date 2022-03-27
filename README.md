<div align="center">
  <h1><code>pokeapi-rs</code></h1>

  <p>
    <strong>Rust bindings/wrapper and client for PokéAPI v2.</strong>
  </p>
</div>

WIP with future support for request/response/image caching and sync/async clients.

With the `ureq` client:

```toml
[dependencies]
pokeapi = "1"
```

With the `reqwest` client:

```toml
[dependencies.pokeapi]
version = "1"
default-features = false
features = ["client-reqwest"]
```

# Usage

```rs
use pokeapi::Client;

let client = Client::new();

// Get a resource by name:
let pikachu = client.pokemon('pikachu')
// Or by ID:
let pikachu = client.pokemon('25')
```

TODO
- doc comment generation on model types from module name
- add endpoints to model through macro
- native tls - ureq needs `Agent` in `Client`
- cache

- CI for both sync/async and other platforms
