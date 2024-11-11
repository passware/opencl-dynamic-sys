use crate::types::{c_void, intptr_t};

// CLeglImageKHR is an opaque handle to an EGLImage
pub type CLeglImageKHR = *mut c_void;

// CLeglDisplayKHR is an opaque handle to an EGLDisplay
pub type CLeglDisplayKHR = *mut c_void;

// CLeglSyncKHR is an opaque handle to an EGLSync object
pub type CLeglSyncKHR = *mut c_void;

// properties passed to clCreateFromEGLImageKHR
pub type cl_egl_image_properties_khr = intptr_t;
