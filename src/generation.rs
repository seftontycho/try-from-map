use crate::field::Field;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_from_impl(struct_name: &syn::Ident, fields: &[Field]) -> TokenStream {
    let field_idents = fields.iter().map(|f| f.ident());

    let extract_fields = fields
        .iter()
        .map(|field| field.generate_deserialize())
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
