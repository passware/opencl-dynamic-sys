mod errors;
pub use errors::{Error, Result};

pub mod constants;
pub mod types;

mod container;
pub use container::{is_opencl_runtime_available, load_library, OpenClRuntime};
