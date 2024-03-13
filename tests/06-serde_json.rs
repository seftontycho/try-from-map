use try_from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {
    a: String,
    b: Option<String>,
    c: i32,
    #[serde_json]
    d: Vec<f64>,
}

#[test]
fn serde_json() {
    let map = std::collections::HashMap::from([
        ("a".to_string(), "bar".to_string()),
        ("c".to_string(), "42".to_string()),
        ("d".to_string(), "[3.14, 2.71]".to_string()),
    ]);

    let foo = Foo::try_from(map).unwrap();

    println!("{:?}", foo);

    assert_eq!(foo.a, "bar");
    assert_eq!(foo.b, None);
    assert_eq!(foo.c, 42);
    assert_eq!(foo.d, vec![3.14, 2.71]);
}
