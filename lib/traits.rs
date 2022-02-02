pub trait Pure<T> {
    fn pure(self) -> T;
}
impl<T> Pure<Option<T>> for T {
    fn pure(self) -> Option<T> {
        Some(self)
    }
}
impl<T, E> Pure<Result<T, E>> for T {
    fn pure(self) -> Result<T, E> {
        Ok(self)
    }
}
