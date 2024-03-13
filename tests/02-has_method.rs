use try_from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {}

#[test]
fn has_method() {
    let map = std::collections::HashMap::new();
    let foo = Foo::try_from(map);

    println!("{:?}", foo);
}
