use crate::types::{cl_platform::*, *};

pub type cl_dx9_media_adapter_type_khr = cl_uint;
pub type cl_dx9_media_adapter_set_khr = cl_uint;
pub type D3DFORMAT = cl_uint;

pub type IDirect3DSurface9_ptr = *mut c_void;
pub type HANDLE = *mut c_void;

pub type clGetDeviceIDsFromDX9MediaAdapterKHR_t = Option<
    unsafe extern "C" fn(
        platform: cl_platform_id,
        num_media_adapters: cl_uint,
        media_adapter_type: *mut cl_dx9_media_adapter_type_khr,
        media_adapters: *mut c_void,
        media_adapter_set: cl_dx9_media_adapter_set_khr,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,
>;

pub type clCreateFromDX9MediaSurfaceKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        adapter_type: cl_dx9_media_adapter_type_khr,
        surface_info: *mut c_void,
        plane: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clEnqueueAcquireDX9MediaSurfacesKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseDX9MediaSurfacesKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// cl_intel_dx9_media_sharing extension

pub type cl_dx9_device_source_intel = cl_uint;
pub type cl_dx9_device_set_intel = cl_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_dx9_surface_info_khr {
    pub resource: IDirect3DSurface9_ptr,
    pub shared_handle: HANDLE,
}

pub type clGetDeviceIDsFromDX9INTEL_t = Option<
    unsafe extern "C" fn(
        platform: cl_platform_id,
        dx9_device_source: cl_dx9_device_source_intel,
        dx9_object: *mut c_void,
        dx9_device_set: cl_dx9_device_set_intel,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,
>;

pub type clCreateFromDX9MediaSurfaceINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: IDirect3DSurface9_ptr,
        sharedHandle: HANDLE,
        plane: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clEnqueueAcquireDX9ObjectsINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseDX9ObjectsINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clGetSupportedDX9MediaSurfaceFormatsINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        plane: cl_uint,
        num_entries: cl_uint,
        dx9_formats: *mut D3DFORMAT,
        num_surface_formats: *mut cl_uint,
    ) -> cl_int,
>;
