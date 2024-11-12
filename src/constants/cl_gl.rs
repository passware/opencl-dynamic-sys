use crate::types::{
    cl_context_properties,
    cl_gl::{cl_gl_context_info, cl_gl_object_type, cl_gl_texture_info},
    cl_platform::{cl_int, cl_uint},
};

pub const CL_GL_OBJECT_BUFFER: cl_gl_object_type = 0x2000;
pub const CL_GL_OBJECT_TEXTURE2D: cl_gl_object_type = 0x2001;
pub const CL_GL_OBJECT_TEXTURE3D: cl_gl_object_type = 0x2002;
pub const CL_GL_OBJECT_RENDERBUFFER: cl_gl_object_type = 0x2003;
pub const CL_GL_OBJECT_TEXTURE2D_ARRAY: cl_gl_object_type = 0x200E;
pub const CL_GL_OBJECT_TEXTURE1D: cl_gl_object_type = 0x200F;
pub const CL_GL_OBJECT_TEXTURE1D_ARRAY: cl_gl_object_type = 0x2010;
pub const CL_GL_OBJECT_TEXTURE_BUFFER: cl_gl_object_type = 0x2011;

pub const CL_GL_TEXTURE_TARGET: cl_gl_texture_info = 0x2004;
pub const CL_GL_MIPMAP_LEVEL: cl_gl_texture_info = 0x2005;
pub const CL_GL_NUM_SAMPLES: cl_gl_texture_info = 0x2012;

// cl_khr_gl_sharing extension
pub const CL_KHR_GL_SHARING: cl_int = 1;

// Additional Error Codes
pub const CL_INVALID_GL_SHAREGROUP_REFERENCE_KHR: cl_int = -1000;

// cl_gl_context_info
pub const CL_CURRENT_DEVICE_FOR_GL_CONTEXT_KHR: cl_gl_context_info = 0x2006;
pub const CL_DEVICES_FOR_GL_CONTEXT_KHR: cl_gl_context_info = 0x2007;

// Additional cl_context_properties
pub const CL_GL_CONTEXT_KHR: cl_context_properties = 0x2008;
pub const CL_EGL_DISPLAY_KHR: cl_context_properties = 0x2009;
pub const CL_GLX_DISPLAY_KHR: cl_context_properties = 0x200A;
pub const CL_WGL_HDC_KHR: cl_context_properties = 0x200B;
pub const CL_CGL_SHAREGROUP_KHR: cl_context_properties = 0x200C;

// cl_khr_gl_event extension
pub const CL_COMMAND_GL_FENCE_SYNC_OBJECT_KHR: cl_uint = 0x200D;
