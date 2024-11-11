use crate::types::{cl_platform::*, *};

pub type cl_gl_object_type = cl_uint;
pub type cl_gl_texture_info = cl_uint;
pub type cl_gl_platform_info = cl_uint;
pub type cl_GLsync = *mut c_void;

// cl_khr_gl_sharing extension
pub type cl_gl_context_info = cl_uint;

pub type clGetGLContextInfoKHR_t = Option<
    unsafe extern "C" fn(
        properties: *const cl_context_properties,
        param_name: cl_gl_context_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clCreateFromGLBuffer_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        bufobj: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateFromGLTexture_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateFromGLRenderbuffer_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        renderbuffer: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clGetGLObjectInfo_t = Option<
    unsafe extern "C" fn(
        memobj: cl_mem,
        gl_object_type: *mut cl_gl_object_type,
        gl_object_name: *mut cl_GLuint,
    ) -> cl_int,
>;

pub type clGetGLTextureInfo_t = Option<
    unsafe extern "C" fn(
        memobj: cl_mem,
        param_name: cl_gl_texture_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clEnqueueAcquireGLObjects_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseGLObjects_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clCreateFromGLTexture2D_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateFromGLTexture3D_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateEventFromGLsyncKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        sync: cl_GLsync,
        errcode_ret: *mut cl_int,
    ) -> cl_event,
>;

pub type clGetSupportedGLTextureFormatsINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        gl_formats: *mut cl_GLenum,
        num_texture_formats: *mut cl_uint,
    ) -> cl_int,
>;
