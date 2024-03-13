Derive `TryFrom<HashMap<String, String>>` for a struct.

Supports structs with named fields that either `impl FromStr` or `serde::Deserialize`.
Fields that implement `serde::Deserialize` can be annotated with `#[serde_json]` to parse the value as JSON.

# Example

```rust
use try_from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {
   a: i32,
   b: f32,
   c: Option<bool>,
   #[serde_json] // Parse as JSON as Vec<f64> does not impl FromStr
   d: Vec<f64>,
}

let map = std::collections::HashMap::from([
    ("a".to_string(), "42".to_string()),
    ("b".to_string(), "3.14".to_string()),
    ("d".to_string(), "[3.14, 2.71]".to_string()),
]);

let foo = Foo::try_from(map).unwrap();
println!("{:?}", foo);

assert_eq!(foo.a, 42);
assert_eq!(foo.b, 3.14);
assert_eq!(foo.c, None);
assert_eq!(foo.d, vec![3.14, 2.71]);
```