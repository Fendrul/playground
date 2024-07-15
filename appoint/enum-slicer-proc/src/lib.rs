use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EnumSlice)]
pub fn derive_enum_iterator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumSlice can only be derived for enums"),
    };

    let variant_idents: Vec<_> = variants
        .iter()
        .map(|v| {
            if let Fields::Unit = v.fields {
                &v.ident
            } else {
                panic!("EnumSlice can only be derived for enums with unit variants")
            }
        })
        .collect();

    let expanded = quote! {
        impl enum_slicer::IntoEnumSlice for #name {
            fn variants_slice<'a>() -> &'a [Self] where Self: Sized {
                &[#(Self::#variant_idents),*]
            }
        }
    };

    TokenStream::from(expanded)
}