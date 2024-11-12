pub use dlopen2::Error;

/// This is a library-specific Result that is returned by all calls to all APIs.
pub type Result<T> = std::result::Result<T, Error>;
