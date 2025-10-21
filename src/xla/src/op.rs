use crate::XlaBuilder;

use cpp::{cpp, cpp_class};

cpp! {{
    #include "xla/client/xla_builder.h"
    #include "xla/client/lib/constants.h"
    #include "xla/client/lib/matrix.h"
    #include "xla/client/lib/math.h"
    #include "xla/statusor.h"
    #include "xla/literal_util.h"
    using namespace xla;
}}

cpp_class!(pub unsafe struct XlaOpRaw as "XlaOp");

#[derive(Clone)]
pub struct XlaOp {
    pub(crate) raw: XlaOpRaw,
    pub(crate) builder: XlaBuilder,
}
