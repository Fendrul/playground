pub trait EnumIterator {
    fn variants_iter() -> impl Iterator<Item = Self>;
}