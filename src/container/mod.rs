#![allow(non_snake_case)]

use dlopen2::wrapper::WrapperApi;

use crate::types::*;

mod utils;
pub use utils::load_library;

#[derive(WrapperApi)]
pub struct OpenCl {
    // Platform API
    clGetPlatformIDs: fn(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int,
}
