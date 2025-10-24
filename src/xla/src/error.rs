use cpp::{cpp, cpp_class};
use cxx::{CxxString, UniquePtr};

cpp! {{
    #include "xla/statusor.h"
    using namespace xla;
}}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected element of type {0}")]
    UnexpectedElementType(i32),

    #[error("not an element type, got: {got:?}")]
    NotAnElementType { got: crate::PrimitiveType },
}

pub type Result<T> = std::result::Result<T, Error>;
