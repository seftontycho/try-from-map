use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod field;
mod generation;

/// Derive `TryFrom<HashMap<String, String>>` for a struct.
///
/// This macro will generate an implementation of `TryFrom<HashMap<String, String>>` for the annotated struct.
/// It will attempt to parse each field from the map, and return an error if any field is missing or cannot be parsed.
/// Fields of type `Option<T>` are supported, and will be set to None if the field is missing from the map.
///
/// Supports structs with named fields that either `impl FromStr` or `serde::Deserialize`.
/// Fields that implement `serde::Deserialize` can be annotated with `#[serde_json]` to parse the value as JSON.
///
/// # Example
///
/// ```rust
/// use try_from_map::TryFromMap;
///
/// #[derive(TryFromMap, Debug)]
/// struct Foo {
///    a: i32,
///    b: f32,
///    c: Option<bool>,
///    #[serde_json] // Parse as JSON as Vec<f64> does not impl FromStr
///    d: Vec<f64>,
/// }
///
///
/// let map = std::collections::HashMap::from([
///     ("a".to_string(), "42".to_string()),
///     ("b".to_string(), "3.14".to_string()),
///     ("d".to_string(), "[3.14, 2.71]".to_string()),
/// ]);
///
/// let foo = Foo::try_from(map).unwrap();
///
/// println!("{:?}", foo);
///
/// assert_eq!(foo.a, 42);
/// assert_eq!(foo.b, 3.14);
/// assert_eq!(foo.c, None);
/// assert_eq!(foo.d, vec![3.14, 2.71]);
///
/// ```
#[proc_macro_derive(TryFromMap, attributes(serde_json))]
pub fn derive_try_from_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    let output = _derive_try_from_map(parsed);

    match output {
        Ok(output) => output.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn _derive_try_from_map(parsed: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = parsed.ident.clone();
    let fields = crate::field::parse_fields(&parsed)?;

    let from_impl = crate::generation::generate_from_impl(&struct_name, &fields);

    Ok(quote! {
        #from_impl
    })
}
