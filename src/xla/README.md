# xla

`xla` is a Rust wrapper around XLA, a compiler for machine learning and linear algebra. The goal of this workspace is to create a set of safe bindings close to XLA's C++ API.

The `vendor` directory contains hand-picked files from `jaxlib` that are needed for minimal functionality instead of bringing in the entire `jaxlib` as a dependency. The `build.rs` script in the workspace root compiles these C++ files into native code that is linked with the Rust library.
