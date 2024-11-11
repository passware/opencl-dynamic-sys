use crate::types::{c_void, cl_uint};

pub type cl_dx9_media_adapter_type_khr = cl_uint;
pub type cl_dx9_media_adapter_set_khr = cl_uint;
pub type D3DFORMAT = cl_uint;

pub type IDirect3DSurface9_ptr = *mut c_void;
pub type HANDLE = *mut c_void;
