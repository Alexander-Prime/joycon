pub trait Has<T> {
    fn has(&self, item: T) -> bool;
}
