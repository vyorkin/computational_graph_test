/// Node expression.
#[derive(Clone)]
pub enum Expr<T> {
    /// Literal of type `T`.
    Literal(T),
    /// Named variable.
    Var(String),
    /// Unary operation.
    Unary(fn(T) -> T),
    /// Binary operation.
    Binary(fn(T, T) -> T),
}

impl<T> From<T> for Expr<T> {
    fn from(x: T) -> Self {
        Expr::Literal(x)
    }
}
