//! [![](https://img.shields.io/crates/v/opencl-dynamic-sys.svg)](https://crates.io/crates/opencl-dynamic-sys)
//! [![](https://docs.rs/opencl-dynamic-sys/badge.svg)](https://docs.rs/opencl-dynamic-sys/)
//! [![OpenCL 3.0](https://img.shields.io/badge/OpenCL-3.0-blue.svg)](https://www.khronos.org/registry/OpenCL/)
//! [![](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://github.com/passware/opencl-dynamic-sys-rs/blob/main/LICENSE)
//! [![Rust](https://github.com/passware/opencl-dynamic-sys-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/passware/opencl-dynamic-sys-rs/actions)
//!
//! Rust library to handle the dynamic load of the OpenCL shared library.
//!
//! ## Description
//!
//! This library in general is a rewritten version of the [opencl-sys-rs](https://github.com/kenba/opencl-sys-rs) library.
//! The main difference is a way in which the OpenCL library will be linked to the program:
//! it will be loaded dynamically in the runtime during the first call to the OpenCL API with the help of the
//! [dlopen2](https://github.com/OpenByteDev/dlopen2) library.
//!
//! If the OpenCL library not found, the error code `CL_RUNTIME_LOAD_FAILED = -2000` will be returned.
//!
//! If the OpenCL library doesn't have some API functions
//! (such as [clCreateBufferWithProperties](https://registry.khronos.org/OpenCL/sdk/3.0/docs/man/html/clCreateBuffer.html),
//! it is v3.0 API function), the error code `CL_FUNCTION_NOT_AVAILABLE = -2001` will be returned.
//!
//! ## Usage
//!
//! You do *not* need to install any OpenCL hardware driver(s) to run your code which depends on `opencl-dynamic-sys` library.
//!
//! Just include the library in the `dependencies` section in your `Cargo.toml` file:
//! ```
//! [dependencies]
//! opencl-dynamic-sys = "0.1"
//! ```
//!
//! ## Search path
//!
//! By default, the OpenCL shared library is searched as follows:
//!
//! - on Windows: `OpenCL.dll`
//! - on macOS: `/System/Library/Frameworks/OpenCL.framework/OpenCL`
//! - otherwise: `libOpenCL.so`
//!
//! If you have the OpenCL shared library in non-standard place, you can use an environment variable `OPENCL_DYLIB_PATH`
//! to define where to look for the library (the value of the variable is a comma-separated string):
//! ```
//! OPENCL_DYLIB_PATH=/usr/lib/libOpenCL.so;/var/lib/OpenCL;%DESKTOP%/OpenCL.dll
//! ```
//!
//! If the OpenCL shared library is found, then the call of `opencl-dynamic-sys::is_opencl_runtime_available()` will return `true`.
//!
//! ## License
//! This library is distributed under the terms of the MIT license, see [LICENSE](https://github.com/passware/opencl-dynamic-sys-rs/blob/main/LICENSE) for details.

mod errors;
pub use errors::{Error, Result};

/// Constants used by the OpenCL API.
pub mod constants;

/// Types used by the OpenCL API.
pub mod types;

mod container;
pub use container::{is_opencl_runtime_available, load_library, OpenCl, OpenClRuntime};
