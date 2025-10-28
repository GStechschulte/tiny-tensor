# xla

_Note_: This is a port of Nox [noxla](https://github.com/elodin-sys/elodin/tree/main/libs/noxla) but is intended to be better documented and more ergonomic (eventually).

`xla` is a Rust wrapper around XLA, a compiler for machine learning and linear algebra. The goal of this workspace is to create a set of safe bindings close to XLA's C++ API.

## Project structure

The `vendor` directory contains hand-picked files from `jaxlib` that are needed for minimal jax-like functionality in Rust instead of bringing in the entire `jaxlib` as a dependency.

The `build.rs` script in the workspace root downloads the XLA compiler and the `jaxlib` C++ files into native code that is linked with the Rust library. This script is also responsible for determining the OS and architecture, and downloading the XLA extension (either shared or as a static binary).

To check if `build.rs` ran properly:

```bash
# 1. Ensure a fresh build
cargo clean

# 2. Enable verbose output and build
cargo build -vv

# 3. Examine the build artifacts
ls -la target/debug/build/xla-*

# 4. Check the out directory for XLA extension
ls -la target/debug/build/xla-*/out/xla_extension

# 5. Look at build output
cat target/debug/build/xla-*/output

# 6. Check compiled library linkage
otool -L target/debug/libxla.dylib
# On Linux:
# ldd target/debug/libxla.so

# 7. Check if custom kernels were compiled
find target/debug/build/xla-* -name "*.o"
```

## Bindings

A set of safe bindings are written using `cpp` and `cxx` crates.

The pattern of defining `cpp` macro blocks

```Rust
cpp! {{
    #include "xla/client/xla_builder.h"
    using namespace xla;
}}
```

including header file(s) and importing the `xla` namespace results in the C++ code being compiled inline with the Rust code. Then, a `cpp_class` macro block

```Rust
cpp_class!(pub unsafe struct XlaBuilder as "std::shared_ptr<XlaBuilder>");
```

creates a Rust type that wraps `std::shared_ptr<XlaBuilder>"`. `unsafe` is necessary because the FFI boundary cannot provide any safety guarantees on the C++ code. However, the `cpp` crate automatically adds the `Drop` trait for C++ types that have destructors.

The goal is to essentially create a set of wrappers around the XLA C++ API so you, the Rust developer, can develop in Rust without having to think about the FFI bounary. When you call methods on the wrappers, you are actually calling into the underlying C++ code. For example, in the following code, the goal is to create a wrapper around the `XlaBuilder` class.

```Rust
cpp! {{
    #include "xla/client/xla_builder.h"
    using namespace xla;
}}

cpp_class!(pub unsafe struct XlaBuilder as "std::shared_ptr<XlaBuilder>");

impl XlaBuilder {
    pub fn new(name: &str) -> Self {
        let_cxx_string!(name = name);
        unsafe {
            // Captures `name` and declares as pointer to string in C++
            // Returns shared pointer to `XlaBuilder` which is converted back to Rust `XlaBuilder`
            cpp!( [name as "std::string*"] -> XlaBuilder as "std::shared_ptr<XlaBuilder>" {
                std::shared_ptr<XlaBuilder> builder(new XlaBuilder(*name));
                return builder;
            })
        }
    }
}
```





## Types

There are multiple interconnected type representations:

* **Native**. Trait that bridges between Rust and XLA types.
* **Element**. Rust types (user-facing).
* **Primitive**. XLA types.

Primitive types are what gets passed across the FFI boundary. Element types are user-facing Rust types. The native type trait connects Rust types to the corresponding XLA types allowing users to use Rust types such as `f32`, `i32` and the native type trait handles the conversion.
