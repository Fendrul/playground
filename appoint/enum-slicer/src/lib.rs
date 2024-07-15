pub trait IntoEnumSlice {
    fn variants_slice<'a>() -> &'a[Self] where Self: Sized;
}