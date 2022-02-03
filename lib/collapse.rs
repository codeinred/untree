/// Allows a type family to be collapsed into a single type
pub trait Collapse<To> {
    fn collapse(self) -> To;
}
