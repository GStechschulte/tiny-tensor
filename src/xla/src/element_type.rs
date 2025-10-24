use std::str::FromStr;

use crate::error::{Error, Result};

/// Primitive types supported by XLA.
///
/// Primitive types represent the underlying XLA enum representation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)] // Match C++ representation
pub enum PrimitiveType {
    Invalid = 0,
    F32 = 1,
    F64 = 2,
    Tuple = 3,
    OpaqueType = 4,
    Token = 5,
}

impl PrimitiveType {
    /// Convert the primitive type to the corresponding element type.
    pub fn element_type(self) -> Result<ElementType> {
        match self {
            Self::F32 => Ok(ElementType::F32),
            Self::F64 => Ok(ElementType::F64),
            Self::Invalid | Self::Tuple | Self::OpaqueType | Self::Token => {
                Err(Error::NotAnElementType { got: self })
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ElementType {
    F32,
    F64,
}

impl FromStr for ElementType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "float32" => Ok(Self::F32),
            "float64" => Ok(Self::F64),
            _ => Err(()),
        }
    }
}

impl ElementType {
    /// Convert the element type to the corresponding primitive type.
    pub fn primitive_type(&self) -> PrimitiveType {
        match self {
            Self::F32 => PrimitiveType::F32,
            Self::F64 => PrimitiveType::F64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_type_from_str() {
        assert_eq!(ElementType::from_str("float32"), Ok(ElementType::F32));
        assert_eq!(ElementType::from_str("invalid"), Err(()));
    }

    #[test]
    fn test_element_to_primitive_type() {
        assert_eq!(ElementType::F32.primitive_type(), PrimitiveType::F32);
    }
}
