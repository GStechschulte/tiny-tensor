//! Bridges Rust types to XLA

use cpp::cpp;

use crate::{XlaBuilder, XlaOp, XlaOpRaw};

cpp! {{
    #include "xla/client/xla_builder.h"
    #include "xla/client/lib/constants.h"
    #include "xla/client/lib/matrix.h"
    #include "xla/statusor.h"
    #include "xla/literal_util.h"
    #include "xla/pjrt/pjrt_api.h"
    #include "xla/pjrt/pjrt_c_api_client.h"
    #include "xla/pjrt/pjrt_client.h"
    using namespace xla;
}}

/// A type implementing the `NativeType` trait can be directly converted to constant ops
/// or literals.
pub trait NativeType: Copy {
    /// Calls `ConstantR0` creating a scalar constant.
    ///
    /// This scalar constant is represented as an `XlaOp` in the computational graph.
    fn constant_r0(builder: &XlaBuilder, value: Self) -> XlaOp;
    // Creates a vector constant.
    // fn constant_r1(builder: &XlaBuilder, val: Self) -> XlaOp;
}

impl NativeType for f64 {
    fn constant_r0(builder: &XlaBuilder, value: Self) -> XlaOp {
        let raw = unsafe {
            cpp!([builder as "std::shared_ptr<XlaBuilder>*", value as "double"] ->XlaOpRaw as "XlaOp" {
                return XlaOp(ConstantR0<double>(builder->get(), value));
            })
        };
        XlaOp {
            raw,
            builder: builder.clone(),
        }
    }
}

impl NativeType for f32 {
    fn constant_r0(builder: &XlaBuilder, value: Self) -> XlaOp {
        let raw = unsafe {
            cpp!([builder as "std::shared_ptr<XlaBuilder>*", value as "float"] ->XlaOpRaw as "XlaOp" {
                return XlaOp(ConstantR0<double>(builder->get(), value));
            })
        };
        XlaOp {
            raw,
            builder: builder.clone(),
        }
    }
}
