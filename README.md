Derive `TryFrom<HashMap<String, String>>` for a struct.

This macro will generate an implementation of `TryFrom<HashMap<String, String>>` for the annotated struct.
It will attempt to parse each field from the map, and return an error if any field is missing or cannot be parsed.
Fields of type `Option<T>` are supported, and will be set to `None` if the field is missing from the map.

Currently, only supports structs with named fields that implement `FromStr`.
Accepting all types that implement `serde::Deserialize` is a future goal.

# Example
```rust
use try_from_map::TryFromMap;
#[derive(TryFromMap, Debug)]
struct Foo {
   a: i32,
   b: f32,   c: Option<bool>,
}

let map = std::collections::HashMap::from([
    ("a".to_string(), "42".to_string()),
    ("b".to_string(), "3.14".to_string()),
]);

let foo = Foo::try_from(map).unwrap();

println!("{:?}", foo);

assert_eq!(foo.a, 42);
assert_eq!(foo.b, 3.14);
assert_eq!(foo.c, None);
```