use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EnumIterator)]
pub fn derive_enum_iterator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumIterator can only be derived for enums"),
    };

    let variant_idents: Vec<_> = variants
        .iter()
        .map(|v| {
            if let Fields::Unit = v.fields {
                &v.ident
            } else {
                panic!("EnumIterator can only be derived for enums with unit variants")
            }
        })
        .collect();

    let expanded = quote! {
        impl enum_iterator::EnumIterator for #name {
            fn variants_iter() -> impl Iterator<Item = Self> {
                [#(Self::#variant_idents),*].into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}