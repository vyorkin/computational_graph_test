use crate::compute::Compute;
use crate::expr::Expr;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};

pub(crate) type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub(crate) type NodeWeakRef<T> = Weak<RefCell<Node<T>>>;

/// Represents a graph node.
#[derive(Clone)]
pub struct Node<T> {
    inputs: Vec<NodeRef<T>>,
    outputs: Vec<NodeWeakRef<T>>,
    expr: Expr<T>,
    pub(crate) value: Option<T>,
}

impl<T: Copy> Node<T> {
    /// Creates a new node that has no inputs.
    pub(crate) fn input(expr: Expr<T>) -> NodeRef<T> {
        Self::new(expr, vec![])
    }

    /// Creates a new node.
    fn new(expr: Expr<T>, inputs: Vec<NodeRef<T>>) -> NodeRef<T> {
        Node {
            inputs,
            outputs: vec![],
            expr,
            value: None,
        }
        .setup()
    }

    fn setup(self) -> NodeRef<T> {
        let node = Rc::new(RefCell::new(self));
        for input in node.borrow().inputs.iter() {
            let output = Rc::downgrade(&node);
            input.borrow_mut().outputs.push(output);
        }
        node
    }

    /// Creates a new unary node.
    pub(crate) fn unary(a: NodeRef<T>, f: fn(T) -> T) -> NodeRef<T> {
        Self::new(Expr::Unary(f), vec![a])
    }

    /// Creates a new binary node.
    pub(crate) fn binary(a: NodeRef<T>, b: NodeRef<T>, f: fn(T, T) -> T) -> NodeRef<T> {
        Self::new(Expr::Binary(f), vec![a, b])
    }

    /// Invalidates cache.
    pub fn invalidate(&mut self) {
        self.value = None;
        self.invalidate_outputs()
    }

    fn invalidate_outputs(&self) {
        for node in &self.outputs {
            if let Some(rc) = node.upgrade() {
                rc.borrow_mut().invalidate()
            }
        }
    }

    fn eval(&self) -> Option<T> {
        match self.expr {
            Expr::Literal(x) => Some(x),
            Expr::Var(_) => self.value,
            Expr::Unary(f) => self.inputs[0].compute().map(f),
            Expr::Binary(f) => {
                let x = self.inputs[0].compute();
                let y = self.inputs[1].compute();
                if let (Some(x), Some(y)) = (x, y) {
                    Some(f(x, y))
                } else {
                    None
                }
            }
        }
    }
}

impl<T: Copy> Compute<T> for NodeRef<T> {
    fn compute(&self) -> Option<T> {
        let mut node = self.borrow_mut();
        if let Some(x) = node.value {
            Some(x)
        } else {
            node.value = node.eval();
            node.value
        }
    }
}
