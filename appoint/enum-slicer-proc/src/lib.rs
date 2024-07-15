use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// This macro derives an implementation of the `IntoEnumSlice` trait for enums with unit variants.
///
/// The `EnumSlice` derive macro can only be applied to enums. It generates an implementation
/// of the `IntoEnumSlice` trait, which provides a method to get a slice of all the enum variants.
///
/// # Example
///
/// ```
/// use enum_slicer_proc::EnumSlice;
/// use enum_slicer::IntoEnumSlice; 
/// 
/// #[derive(EnumSlice, Debug, PartialEq)] 
/// enum MyEnum { 
///     Variant1, 
///     Variant2, 
///     Variant3, 
/// } 
/// 
/// fn main() { 
///    let variants = MyEnum::variants_slice(); 
///    assert_eq!(variants, &[MyEnum::Variant1, MyEnum::Variant2, MyEnum::Variant3]); 
/// } 
/// ```
///
/// # Panics
///
/// This macro will panic if:
/// - It is applied to a non-enum type.
/// - The enum contains non-unit variants (i.e., variants with fields).
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