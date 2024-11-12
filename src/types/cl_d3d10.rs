use crate::types::{cl_platform::*, *};

// cl_khr_d3d10_sharing

pub type cl_d3d10_device_source_khr = cl_uint;
pub type cl_d3d10_device_set_khr = cl_uint;

pub type ID3D10Buffer_ptr = *mut c_void;
pub type ID3D10Texture2D_ptr = *mut c_void;
pub type ID3D10Texture3D_ptr = *mut c_void;

pub type clGetDeviceIDsFromD3D10KHR_t = Option<
    unsafe extern "C" fn(
        platform: cl_platform_id,
        d3d_device_source: cl_d3d10_device_source_khr,
        d3d_object: *mut c_void,
        d3d_device_set: cl_d3d10_device_set_khr,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,
>;

pub type clCreateFromD3D10BufferKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Buffer_ptr,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateFromD3D10Texture2DKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Texture2D_ptr,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clCreateFromD3D10Texture3DKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Texture3D_ptr,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clEnqueueAcquireD3D10ObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseD3D10ObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// when cl_khr_d3d10_sharing is supported

pub type clGetSupportedD3D10TextureFormatsINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        d3d10_formats: *mut DXGI_FORMAT,
        num_surface_formats: *mut cl_uint,
    ) -> cl_int,
>;
