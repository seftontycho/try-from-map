use quote::quote;

pub(crate) struct Field {
    name: syn::Ident,
    is_optional: bool,
    serde_json: bool,
}

impl Field {
    pub(crate) fn generate_deserialize(&self) -> proc_macro2::TokenStream {
        let ident = &self.name;
        let ident_str = format!("{}", ident);

        let err_msg = format!("Field {} not found", ident);

        let parse = if self.serde_json {
            quote! {
                serde_json::from_str(&value).unwrap()
            }
        } else {
            quote! {
                value.parse().unwrap()
            }
        };

        if self.is_optional {
            quote! {
                let #ident = map.get(#ident_str).map(|value| {
                    #parse
                });
            }
        } else {
            quote! {
                let #ident = map.get(#ident_str).map(|value| {
                    #parse
                }).ok_or_else(|| #err_msg)?;
            }
        }
    }

    pub(crate) fn ident(&self) -> &syn::Ident {
        &self.name
    }
}

fn parse_field(field: &syn::Field) -> Field {
    let name = field.ident.clone().unwrap();
    let is_optional = is_optional_field(field);
    let serde_json = has_attr(&field.attrs, "serde_json");

    Field {
        name,
        is_optional,
        serde_json,
    }
}

pub(crate) fn parse_fields(parsed: &syn::DeriveInput) -> syn::Result<Vec<Field>> {
    match &parsed.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => Ok(fields.named.iter().map(parse_field).collect()),
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

pub(crate) fn is_optional_field(field: &syn::Field) -> bool {
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

fn has_attr(attrs: &[syn::Attribute], attr_name: &str) -> bool {
    attrs.iter().any(|attr| {
        let path = &attr.path();
        let segments = &path.segments;
        let segment = segments.first().unwrap();
        segment.ident == attr_name
    })
}
