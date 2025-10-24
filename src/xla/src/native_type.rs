//! Bridges Rust types to XLA

use crate::{ElementType, PrimitiveType, XlaBuilder, XlaOp};

/// A type implementing the `NativeType` trait can be directly converted to constant ops
/// or literals.
pub trait NativeType: Sized {
    const ELEMENT_TYPE: ElementType;
    const PRIMITIVE_TYPE: PrimitiveType;

    /// Creates a scalar constant.
    fn constant_r0(builder: &XlaBuilder, val: Self) -> XlaOp;
    /// Creates a vector constant.
    fn constant_r1(builder: &XlaBuilder, val: Self) -> XlaOp;
}
