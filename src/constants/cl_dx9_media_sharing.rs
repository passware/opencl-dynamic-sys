use crate::types::{
    cl_command_type, cl_context_info,
    cl_dx9_media_sharing::{cl_dx9_media_adapter_set_khr, cl_dx9_media_adapter_type_khr},
    cl_image_info, cl_mem_info,
    cl_platform::cl_int,
};

pub const CL_INVALID_DX9_MEDIA_ADAPTER_KHR: cl_int = -1010;
pub const CL_INVALID_DX9_MEDIA_SURFACE_KHR: cl_int = -1011;
pub const CL_DX9_MEDIA_SURFACE_ALREADY_ACQUIRED_KHR: cl_int = -1012;
pub const CL_DX9_MEDIA_SURFACE_NOT_ACQUIRED_KHR: cl_int = -1013;

// cl_media_adapter_type_khr
pub const CL_ADAPTER_D3D9_KHR: cl_dx9_media_adapter_type_khr = 0x2020;
pub const CL_ADAPTER_D3D9EX_KHR: cl_dx9_media_adapter_type_khr = 0x2021;
pub const CL_ADAPTER_DXVA_KHR: cl_dx9_media_adapter_type_khr = 0x2022;

// cl_media_adapter_set_khr
pub const CL_PREFERRED_DEVICES_FOR_DX9_MEDIA_ADAPTER_KHR: cl_dx9_media_adapter_set_khr = 0x2023;
pub const CL_ALL_DEVICES_FOR_DX9_MEDIA_ADAPTER_KHR: cl_dx9_media_adapter_set_khr = 0x2024;

// cl_context_info
pub const CL_CONTEXT_ADAPTER_D3D9_KHR: cl_context_info = 0x2025;
pub const CL_CONTEXT_ADAPTER_D3D9EX_KHR: cl_context_info = 0x2026;
pub const CL_CONTEXT_ADAPTER_DXVA_KHR: cl_context_info = 0x2027;

// cl_mem_info
pub const CL_MEM_DX9_MEDIA_ADAPTER_TYPE_KHR: cl_mem_info = 0x2028;
pub const CL_MEM_DX9_MEDIA_SURFACE_INFO_KHR: cl_mem_info = 0x2029;

// cl_image_info
pub const CL_IMAGE_DX9_MEDIA_PLANE_KHR: cl_image_info = 0x202A;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_DX9_MEDIA_SURFACES_KHR: cl_command_type = 0x202B;
pub const CL_COMMAND_RELEASE_DX9_MEDIA_SURFACES_KHR: cl_command_type = 0x202C;
