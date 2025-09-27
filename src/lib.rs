use std::sync::Arc;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct ExprId(usize);

impl Default for ExprId {
    /// Provides default generation of unique identifiers for expressions.
    fn default() -> Self {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        Self(COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

/// Represents different types of nodes in the expression tree.
///
/// Leaf nodes contain data whereas internal nodes contain other
/// expressions.
#[derive(Debug)]
enum ExprNode {
    Constant { value: f64 },
    Parameter { index: u32, name: String },
    Add { lhs: Expr, rhs: Expr },
}

/// Core structure for representing computational expressions.
///
/// Multiple expressions can share the same sub-expression allowing computations
/// to reuse intermediate results.
#[derive(Debug)]
struct Expr {
    node: Arc<ExprNode>,
    id: ExprId,
}

impl Expr {
    fn new(node: ExprNode) -> Self {
        Self {
            node: Arc::new(node),
            id: ExprId::default(),
        }
    }
    fn parameter(index: u32, name: String) -> Self {
        Self::new(ExprNode::Parameter { index, name })
    }

    fn constant(value: f64) -> Self {
        Self::new(ExprNode::Constant { value })
    }

    fn add(lhs: Expr, rhs: Expr) -> Self {
        Self::new(ExprNode::Add { lhs, rhs })
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::add(self, rhs)
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::constant(value)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expr = Expr::add(Expr::constant(2.0), Expr::constant(4.0));

        let a = Expr::new(ExprNode::Constant { value: 2.0 });
        let b = Expr::new(ExprNode::Constant { value: 4.0 });
        let expr = a + b;

        let x = Expr::parameter(0, "x".to_string());
        let y = Expr::parameter(1, "y".to_string());

        let expr = (x + 2f64.into()) + (y + 1f64.into());
    }
}
