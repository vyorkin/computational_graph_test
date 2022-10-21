/// Trait for "computable" types.
pub trait Compute<T> {
    fn compute(&self) -> Option<T>;
}
