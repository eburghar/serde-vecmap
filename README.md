Loosely based on [serde-tuple-vec-map](https://github.com/daboross/serde-tuple-vec-map)

Deserialize maps or JSON objects in serde to a vec of tuples rather than a HashMap for when you're only ever going to iterate over the result.

Usage:

```rust
// replace this:
#[derive(Serialize, Deserialize)]
struct MyStuff {
    data: HashMap<KeyType, ValueType>,
}
```

```rust
use serde_vecmap:vecmap;

// with this:
#[derive(Serialize, Deserialize)]
struct MyStuff {
    #[serde(with = "vecmap")]
    data: Vec<(KeyType, ValueType)>,
}
```

if you need an optional map

```rust
use serde_vecmap:opt_vecmap;

#[derive(Serialize, Deserialize)]
struct MyStuff {
    #[serde(with = "opt_vecmap")]
    data: Option<Vec<(KeyType, ValueType)>>,
}
```
