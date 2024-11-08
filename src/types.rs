#![allow(non_camel_case_types, non_upper_case_globals)]

use libc::{c_void, intptr_t};

// cl_platform.h

pub type cl_char = i8;
pub type cl_uchar = u8;
pub type cl_short = i16;
pub type cl_ushort = u16;
pub type cl_int = i32;
pub type cl_uint = u32;
pub type cl_long = i64;
pub type cl_ulong = u64;

pub type cl_half = u16;
pub type cl_float = f32;
pub type cl_double = f64;

pub type cl_GLuint = u32;
pub type cl_GLint = i32;
pub type cl_GLenum = u32;
pub type DXGI_FORMAT = u32;

// cl.h

pub type cl_platform_id = *mut c_void;
pub type cl_device_id = *mut c_void;
pub type cl_context = *mut c_void;
pub type cl_command_queue = *mut c_void;
pub type cl_mem = *mut c_void;
pub type cl_program = *mut c_void;
pub type cl_kernel = *mut c_void;
pub type cl_event = *mut c_void;
pub type cl_sampler = *mut c_void;

pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_properties = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_device_fp_config = cl_bitfield;
pub type cl_device_mem_cache_type = cl_uint;
pub type cl_device_local_mem_type = cl_uint;
pub type cl_device_exec_capabilities = cl_bitfield;
pub type cl_device_svm_capabilities = cl_bitfield;
pub type cl_command_queue_properties = cl_bitfield;
pub type cl_device_partition_property = intptr_t;
pub type cl_device_affinity_domain = cl_bitfield;

pub type cl_context_properties = intptr_t;
pub type cl_context_info = cl_uint;
pub type cl_queue_properties = cl_properties;
pub type cl_command_queue_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
pub type cl_svm_mem_flags = cl_bitfield;
pub type cl_mem_object_type = cl_uint;
pub type cl_mem_info = cl_uint;
pub type cl_mem_migration_flags = cl_bitfield;
pub type cl_image_info = cl_uint;
pub type cl_buffer_create_type = cl_uint;
pub type cl_addressing_mode = cl_uint;
pub type cl_filter_mode = cl_uint;
pub type cl_sampler_info = cl_uint;
pub type cl_map_flags = cl_bitfield;
pub type cl_pipe_properties = intptr_t;
pub type cl_pipe_info = cl_uint;
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
pub type cl_program_binary_type = cl_uint;
pub type cl_build_status = cl_int;
pub type cl_kernel_info = cl_uint;
pub type cl_kernel_arg_info = cl_uint;
pub type cl_kernel_arg_address_qualifier = cl_uint;
pub type cl_kernel_arg_access_qualifier = cl_uint;
pub type cl_kernel_arg_type_qualifier = cl_uint;
pub type cl_kernel_work_group_info = cl_uint;
pub type cl_kernel_sub_group_info = cl_uint;
pub type cl_event_info = cl_uint;
pub type cl_command_type = cl_uint;
pub type cl_profiling_info = cl_uint;
pub type cl_sampler_properties = cl_bitfield;
pub type cl_kernel_exec_info = cl_uint;
pub type cl_device_atomic_capabilities = cl_bitfield;
pub type cl_device_device_enqueue_capabilities = cl_bitfield;
pub type cl_khronos_vendor_id = cl_uint;
pub type cl_mem_properties = cl_properties;
pub type cl_version = cl_uint;
