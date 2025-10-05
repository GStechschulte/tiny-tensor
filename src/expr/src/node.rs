use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
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
pub enum ExprNode {
    Constant { value: f64 },
    Parameter { index: u32, name: String },
    Add { lhs: Expr, rhs: Expr },
    Mul { lhs: Expr, rhs: Expr },
}

/// Core structure for representing computational expressions.
///
/// Multiple expressions can share the same sub-expression allowing computations
/// to reuse intermediate results.
#[derive(Debug)]
pub struct Expr {
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

    fn mul(lhs: Expr, rhs: Expr) -> Self {
        Self::new(ExprNode::Mul { lhs, rhs })
    }
}

/// Dereference and return the node
impl Deref for Expr {
    type Target = ExprNode;

    fn deref(&self) -> &Self::Target {
        &self.node
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

/// Visitor trait that defines how to process each node type of an expression tree.
pub trait ExprVisitor<T> {
    fn visit_constant(&mut self, value: f64) -> T;
    fn visit_parameter(&mut self, index: u32, name: &str) -> T;
    fn visit_add(&mut self, lhs: T, rhs: T) -> T;
    fn visit_mul(&mut self, lhs: T, rhs: T) -> T;
}

impl Expr {
    pub fn id(&self) -> ExprId {
        self.id
    }

    pub fn walk<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match &*self.node {
            ExprNode::Constant { value } => visitor.visit_constant(*value),
            ExprNode::Parameter { index, name } => visitor.visit_parameter(*index, name),
            ExprNode::Add { lhs, rhs } => {
                let lhs_result = lhs.walk(visitor);
                let rhs_result = rhs.walk(visitor);
                visitor.visit_add(lhs_result, rhs_result)
            }
            ExprNode::Mul { lhs, rhs } => {
                let lhs_result = lhs.walk(visitor);
                let rhs_result = rhs.walk(visitor);
                visitor.visit_mul(lhs_result, rhs_result)
            }
        }
    }
}

pub struct Tracer {
    context: HashMap<u32, f64>,
}

impl ExprVisitor<f64> for Tracer {
    fn visit_constant(&mut self, value: f64) -> f64 {
        value
    }

    fn visit_parameter(&mut self, index: u32, name: &str) -> f64 {
        self.context
            .get(&index)
            .copied()
            .unwrap_or_else(|| panic!("Parameter {} ('{}') not set", index, name))
    }

    fn visit_add(&mut self, lhs: f64, rhs: f64) -> f64 {
        lhs + rhs
    }

    fn visit_mul(&mut self, lhs: f64, rhs: f64) -> f64 {
        lhs * rhs
    }
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn visit(&mut self, expr: &Expr) -> f64 {
        expr.walk(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_add_expr() {
        let expr = Expr::add(Expr::constant(2.0), Expr::constant(4.0));

        let a = Expr::new(ExprNode::Constant { value: 2.0 });
        let b = Expr::new(ExprNode::Constant { value: 4.0 });
        let expr = a + b;

        let x = Expr::parameter(0, "x".to_string());
        let y = Expr::parameter(1, "y".to_string());

        let expr = (x + 2f64.into()) + (y + 1f64.into());
        println!("{:?}", expr);
    }

    #[test]
    fn build_mul_expr() {
        let expr = Expr::mul(Expr::constant(2.0), Expr::constant(3.0));
        println!("{:?}", expr);
    }

    #[test]
    fn eval_add() {
        let expr = Expr::add(Expr::constant(5.0), Expr::constant(5.0));
        let mut tracer = Tracer::new();
        let result = tracer.visit(&expr);
        println!("{}", result);
    }

    #[test]
    fn eval_mul() {
        let expr = Expr::mul(Expr::constant(2.0), Expr::constant(3.0));
        let mut tracer = Tracer::new();
        let result = tracer.visit(&expr);
        println!("{}", result);
    }
}
