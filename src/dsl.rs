use num_traits::{Float, Pow};
use std::ops::{Add, Mul};

pub use crate::Compute;
use crate::{expr::Expr, node::*, Var};

pub fn lit<T: Copy>(x: T) -> NodeRef<T> {
    Node::input(x.into())
}

pub fn var<T: Copy>(name: &str) -> Var<T> {
    Var(Node::input(Expr::Var(String::from(name))))
}

pub fn add<T: Add<Output = T> + Copy>(a: NodeRef<T>, b: NodeRef<T>) -> NodeRef<T> {
    Node::binary(a, b, |x, y| x + y)
}

pub fn mul<T: Mul<Output = T> + Copy>(a: NodeRef<T>, b: NodeRef<T>) -> NodeRef<T> {
    Node::binary(a, b, |x, y| x * y)
}

pub fn sin<T: Float>(x: NodeRef<T>) -> NodeRef<T> {
    Node::unary(x, |x| x.sin())
}

pub fn pow<T: Pow<T, Output = T> + Copy>(a: NodeRef<T>, b: NodeRef<T>) -> NodeRef<T> {
    Node::binary(a, b, |x, y| x.pow(y))
}
