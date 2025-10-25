use cpp::{cpp, cpp_class};
use cxx::let_cxx_string;

use crate::{NativeType, XlaOp};

cpp! {{
    #include "xla/client/xla_builder.h"
    using namespace xla;
}}

cpp_class!(pub unsafe struct XlaBuilder as "std::shared_ptr<XlaBuilder>");

impl XlaBuilder {
    pub fn new(name: &str) -> Self {
        let_cxx_string!(name = name);
        unsafe {
            cpp!( [name as "std::string*"] -> XlaBuilder as "std::shared_ptr<XlaBuilder>" {
                std::shared_ptr<XlaBuilder> builder(new XlaBuilder(*name));
                return builder;
            })
        }
    }

    pub fn constant<T: NativeType>(&self, val: T) -> XlaOp {
        T::constant_r0(self, val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_constants() {
        let builder = XlaBuilder::new("test");
        builder.constant(3.14f64);
        builder.constant(3.14f32);
    }
}
