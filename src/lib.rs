mod errors;
pub use errors::{Error, Result};

pub mod types;

mod container;
pub use container::{load_library, OpenClRuntime};
