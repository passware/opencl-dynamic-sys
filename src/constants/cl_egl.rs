use crate::types::{cl_command_type, cl_platform::cl_int};

// Command type for events created with clEnqueueAcquireEGLObjectsKHR
pub const CL_COMMAND_EGL_FENCE_SYNC_OBJECT_KHR: cl_command_type = 0x202F;
pub const CL_COMMAND_ACQUIRE_EGL_OBJECTS_KHR: cl_command_type = 0x202D;
pub const CL_COMMAND_RELEASE_EGL_OBJECTS_KHR: cl_command_type = 0x202E;

// Error type for clCreateFromEGLImageKHR
pub const CL_INVALID_EGL_OBJECT_KHR: cl_int = -1093;
pub const CL_EGL_RESOURCE_NOT_ACQUIRED_KHR: cl_int = -1092;
