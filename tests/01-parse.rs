use try_from_map::TryFromMap;

#[derive(TryFromMap)]
struct Foo {
    bar: String,
    baz: String,
}

#[test]
fn parse() {}
