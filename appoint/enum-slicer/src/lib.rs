/// A trait for converting an enum into a slice of all of its variants.
///
/// This trait provides a method to get a slice containing all the variants of an enum.
/// It can be useful for iterating over all possible values of an enum.
///
/// # Example
///
/// ```
/// use crate::enum_slicer::IntoEnumSlice;
///
/// #[derive(Debug)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
///
/// impl IntoEnumSlice for Color {
///     fn variants_slice<'a>() -> &'a [Self]
///     where
///         Self: Sized,
///     {
///         static VARIANTS: &[Color] = &[Color::Red, Color::Green, Color::Blue];
///         VARIANTS
///     }
/// }
///
/// let colors = Color::variants_slice();
/// for color in colors {
///     println!("{:?}", color);
/// }
///```
pub trait IntoEnumSlice {
    /// Returns a slice containing all the variants of the enum.
    fn variants_slice<'a>() -> &'a [Self]
    where
        Self: Sized;
}
