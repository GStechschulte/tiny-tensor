use cpp::{cpp, cpp_class};
use cxx::let_cxx_string;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        XlaBuilder::new("test");
    }
}
