use crate::node::NodeRef;

/// Represents a variable graph node.
#[derive(Clone)]
pub struct Var<T>(pub(crate) NodeRef<T>);

impl<T: Copy> Var<T> {
    /// Sets a new value and invalidates cache.
    pub fn set(&self, x: T) {
        self.clear();
        self.0.borrow_mut().value = Some(x);
    }

    /// Clears the value by invalidating cache.
    pub fn clear(&self) {
        self.0.borrow_mut().invalidate()
    }
}

impl<T> From<Var<T>> for NodeRef<T> {
    fn from(var: Var<T>) -> Self {
        var.0
    }
}
