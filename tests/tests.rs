#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-has_method.rs");
    t.pass("tests/03-strings_only.rs");
    t.pass("tests/04-optional_fields.rs");
}
