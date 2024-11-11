use crate::types::{cl_icd::*, cl_platform::*, *};

// cl_loader_layers
pub type cl_layer_info = cl_uint;
pub type cl_layer_api_version = cl_uint;

pub type clGetLayerInfo_t = Option<
    unsafe extern "C" fn(
        param_name: cl_layer_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clInitLayer_t = Option<
    unsafe extern "C" fn(
        num_entries: cl_uint,
        target_dispatch: *const cl_icd_dispatch,
        num_entries_ret: *mut cl_uint,
        layer_dispatch_ret: *mut *const cl_icd_dispatch,
    ) -> cl_int,
>;
