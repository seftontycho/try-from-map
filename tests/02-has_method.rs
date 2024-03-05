use try_from_map::TryFromMap;

#[derive(TryFromMap, Debug)]
struct Foo {
    bar: String,
    baz: String,
}

fn main() {
    let map = std::collections::HashMap::new();
    let foo = Foo::try_from(map);

    println!("{:?}", foo);
}
