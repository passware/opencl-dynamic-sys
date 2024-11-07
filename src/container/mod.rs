use dlopen2::wrapper::WrapperApi;

mod utils;
pub use utils::load_library;

#[derive(WrapperApi)]
pub struct OpenCl {}
