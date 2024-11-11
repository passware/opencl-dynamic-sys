use crate::types::{c_void, cl_platform::cl_uint};

pub type cl_gl_object_type = cl_uint;
pub type cl_gl_texture_info = cl_uint;
pub type cl_gl_platform_info = cl_uint;
pub type cl_GLsync = *mut c_void;

// cl_khr_gl_sharing extension
pub type cl_gl_context_info = cl_uint;
