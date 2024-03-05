use from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {
    bar: String,
    baz: Option<String>,
}

fn main() {
    let map = std::collections::HashMap::from([("bar".to_string(), "bar".to_string())]);

    let foo = Foo::try_from(map).unwrap();

    println!("{:?}", foo);

    assert_eq!(foo.bar, "bar");
    assert_eq!(foo.baz, None);
}
