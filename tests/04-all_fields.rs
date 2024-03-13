use try_from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {
    bar: i32,
    baz: f32,
}

#[test]
fn all_fields() {
    let map = std::collections::HashMap::from([
        ("bar".to_string(), "-52".to_string()),
        ("baz".to_string(), "6.9".to_string()),
    ]);

    let foo = Foo::try_from(map).unwrap();

    println!("{:?}", foo);

    assert_eq!(foo.bar, -52_i32);
    assert_eq!(foo.baz, 6.9_f32);
}
