use crate::types::{cl_d3d10::*, cl_platform::*, *};

// cl_khr_d3d10_sharing

// Error Codes
pub const CL_INVALID_D3D10_DEVICE_KHR: cl_int = -1002;
pub const CL_INVALID_D3D10_RESOURCE_KHR: cl_int = -1003;
pub const CL_D3D10_RESOURCE_ALREADY_ACQUIRED_KHR: cl_int = -1004;
pub const CL_D3D10_RESOURCE_NOT_ACQUIRED_KHR: cl_int = -1005;

// cl_d3d10_device_source_khr
pub const CL_D3D10_DEVICE_KHR: cl_d3d10_device_source_khr = 0x4010;
pub const CL_D3D10_DXGI_ADAPTER_KHR: cl_d3d10_device_source_khr = 0x4011;

// cl_d3d10_device_set_khr
pub const CL_PREFERRED_DEVICES_FOR_D3D10_KHR: cl_d3d10_device_set_khr = 0x4012;
pub const CL_ALL_DEVICES_FOR_D3D10_KHR: cl_d3d10_device_set_khr = 0x4013;

// cl_context_info
pub const CL_CONTEXT_D3D10_DEVICE_KHR: cl_context_info = 0x4014;
pub const CL_CONTEXT_D3D10_PREFER_SHARED_RESOURCES_KHR: cl_context_info = 0x402C;

// cl_mem_info
pub const CL_MEM_D3D10_RESOURCE_KHR: cl_mem_info = 0x4015;

// cl_image_info
pub const CL_IMAGE_D3D10_SUBRESOURCE_KHR: cl_image_info = 0x4016;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_D3D10_OBJECTS_KHR: cl_command_type = 0x4017;
pub const CL_COMMAND_RELEASE_D3D10_OBJECTS_KHR: cl_command_type = 0x4018;
