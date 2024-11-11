use crate::types::{
    cl_command_type, cl_context_info,
    cl_d3d11::{cl_d3d11_device_set_khr, cl_d3d11_device_source_khr},
    cl_image_info, cl_int, cl_mem_info,
};

pub const CL_INVALID_D3D11_DEVICE_KHR: cl_int = -1006;
pub const CL_INVALID_D3D11_RESOURCE_KHR: cl_int = -1007;
pub const CL_D3D11_RESOURCE_ALREADY_ACQUIRED_KHR: cl_int = -1008;
pub const CL_D3D11_RESOURCE_NOT_ACQUIRED_KHR: cl_int = -1009;

pub const CL_D3D11_DEVICE_KHR: cl_d3d11_device_source_khr = 0x4019;
pub const CL_D3D11_DXGI_ADAPTER_KHR: cl_d3d11_device_source_khr = 0x401A;

pub const CL_PREFERRED_DEVICES_FOR_D3D11_KHR: cl_d3d11_device_set_khr = 0x401B;
pub const CL_ALL_DEVICES_FOR_D3D11_KHR: cl_d3d11_device_set_khr = 0x401C;

// cl_context_info
pub const CL_CONTEXT_D3D11_DEVICE_KHR: cl_context_info = 0x401D;
pub const CL_CONTEXT_D3D11_PREFER_SHARED_RESOURCES_KHR: cl_context_info = 0x402D;

// cl_mem_info
pub const CL_MEM_D3D11_RESOURCE_KHR: cl_mem_info = 0x401E;

// cl_image_info
pub const CL_IMAGE_D3D11_SUBRESOURCE_KHR: cl_image_info = 0x401F;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_D3D11_OBJECTS_KHR: cl_command_type = 0x4020;
pub const CL_COMMAND_RELEASE_D3D11_OBJECTS_KHR: cl_command_type = 0x4021;
