use crate::types::{cl_platform::*, *};

// CLeglImageKHR is an opaque handle to an EGLImage
pub type CLeglImageKHR = *mut c_void;

// CLeglDisplayKHR is an opaque handle to an EGLDisplay
pub type CLeglDisplayKHR = *mut c_void;

// CLeglSyncKHR is an opaque handle to an EGLSync object
pub type CLeglSyncKHR = *mut c_void;

// properties passed to clCreateFromEGLImageKHR
pub type cl_egl_image_properties_khr = intptr_t;

pub type clCreateFromEGLImageKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        egldisplay: CLeglDisplayKHR,
        eglimage: CLeglImageKHR,
        flags: cl_mem_flags,
        properties: *const cl_egl_image_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clEnqueueAcquireEGLObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseEGLObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clCreateEventFromEGLSyncKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        sync: CLeglSyncKHR,
        display: CLeglDisplayKHR,
        errcode_ret: *mut cl_int,
    ) -> cl_event,
>;
