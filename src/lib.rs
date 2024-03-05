use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, parse_macro_input, DeriveInput};

#[proc_macro_derive(TryFromMap)]
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
    let fields = parse_fields(&parsed)?;

    let from_impl = generate_from_impl(&struct_name, &fields);

    println!("from_impl: {}", from_impl.to_string());

    Ok(quote! {
        #from_impl
    })
}

fn generate_from_impl(struct_name: &syn::Ident, fields: &[syn::Field]) -> TokenStream {
    let field_idents = fields.iter().map(|f| f.ident.clone().unwrap());

    let extract_fields = fields
        .iter()
        .map(|field_ident| {
            let ident = field_ident.ident.as_ref().unwrap();
            let ident_str = format!("{}", ident);

            if is_optional_field(field_ident) {
                return quote! {
                    let #ident = match map.get(#ident_str) {
                        Some(value) => Some(value.parse()?),
                        None => None,
                    }
                };
            }

            let err_msg = format!("Field {} not found", ident);

            println!("ident_str: {}", ident_str);

            quote! {
                let #ident = map.get(#ident_str).ok_or_else(|| #err_msg)?.parse()?
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl std::convert::TryFrom<std::collections::HashMap<String, String>> for #struct_name {
            type Error = std::boxed::Box<dyn std::error::Error>;

            fn try_from(map: std::collections::HashMap<String, String>) -> Result<Self, Self::Error> {
                #(#extract_fields;)*

                Ok(Self {
                    #(#field_idents,)*
                })
            }
        }
    }
}

fn parse_fields(parsed: &DeriveInput) -> syn::Result<Vec<syn::Field>> {
    match &parsed.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => Ok(fields.named.iter().cloned().collect()),
            _ => Err(syn::Error::new_spanned(
                parsed,
                "Only named fields are supported",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            parsed,
            "Only structs are supported",
        )),
    }
}

fn is_optional_field(field: &syn::Field) -> bool {
    let path_segments = match &field.ty {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path { segments, .. },
        }) => segments,
        _ => return false,
    };

    let segment = match path_segments.first() {
        Some(segment) => segment,
        None => return false,
    };

    if segment.ident != "Option" {
        return false;
    }

    true
}
