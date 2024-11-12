use crate::{
    constants::cl_ext::{CL_NAME_VERSION_MAX_NAME_SIZE_KHR, CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL},
    types::{cl_platform::*, *},
};

// cl_khr_command_buffer
pub type cl_platform_command_buffer_capabilities_khr = cl_bitfield;
pub type cl_device_command_buffer_capabilities_khr = cl_bitfield;
pub type cl_command_buffer_khr = *mut c_void;
pub type cl_sync_point_khr = cl_uint;
pub type cl_command_buffer_info_khr = cl_uint;
pub type cl_command_buffer_state_khr = cl_uint;
pub type cl_command_buffer_properties_khr = cl_properties;
pub type cl_command_buffer_flags_khr = cl_bitfield;
pub type cl_ndrange_kernel_command_properties_khr = cl_properties;
pub type cl_mutable_command_khr = *mut c_void;

pub type clCreateCommandBufferKHR_t = Option<
    unsafe extern "C" fn(
        num_queues: cl_uint,
        queues: *const cl_command_queue,
        properties: *const cl_command_buffer_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_buffer_khr,
>;

pub type clFinalizeCommandBufferKHR_t =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clRetainCommandBufferKHR_t =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clReleaseCommandBufferKHR_t =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clEnqueueCommandBufferKHR_t = Option<
    unsafe extern "C" fn(
        num_queues: cl_uint,
        queues: *mut cl_command_queue,
        command_buffer: cl_command_buffer_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clCommandBarrierWithWaitListKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferRectKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        src_row_pitch: size_t,
        src_slice_pitch: size_t,
        dst_row_pitch: size_t,
        dst_slice_pitch: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferToImageKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyImageKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyImageToBufferKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandFillBufferKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandFillImageKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandNDRangeKernelKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        properties: *const cl_ndrange_kernel_command_properties_khr,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        local_work_size: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandSVMMemcpyKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandSVMMemFillKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clGetCommandBufferInfoKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        param_name: cl_command_buffer_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clRemapCommandBufferKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        automatic: cl_bool,
        num_queues: cl_uint,
        queues: *const cl_command_queue,
        num_handles: cl_uint,
        handles: *const cl_mutable_command_khr,
        handles_ret: *mut cl_mutable_command_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_buffer_khr,
>;

// cl_khr_command_buffer_mutable_dispatch
pub type cl_command_buffer_structure_type_khr = cl_uint;
pub type cl_mutable_dispatch_fields_khr = cl_bitfield;
pub type cl_mutable_command_info_khr = cl_uint;
pub type cl_mutable_dispatch_asserts_khr = cl_bitfield;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mutable_dispatch_arg_khr {
    pub arg_index: cl_uint,
    pub arg_size: size_t,
    pub arg_value: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mutable_dispatch_exec_info_khr {
    pub param_name: cl_uint,
    pub param_value_size: size_t,
    pub param_value: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mutable_dispatch_config_khr {
    pub t_type: cl_command_buffer_structure_type_khr,
    pub next: *const c_void,
    pub command: cl_mutable_command_khr,
    pub num_args: cl_uint,
    pub num_svm_args: cl_uint,
    pub num_exec_infos: cl_uint,
    pub work_dim: cl_uint,
    pub arg_list: *const cl_mutable_dispatch_arg_khr,
    pub arg_svm_list: *const cl_mutable_dispatch_arg_khr,
    pub exec_info_list: *const cl_mutable_dispatch_exec_info_khr,
    pub global_work_offset: *const size_t,
    pub global_work_size: *const size_t,
    pub local_work_size: *const size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mutable_base_config_khr {
    pub t_type: cl_command_buffer_structure_type_khr,
    pub next: *const c_void,
    pub num_mutable_dispatch: cl_uint,
    pub mutable_dispatch_list: *const cl_mutable_dispatch_config_khr,
}

pub type clUpdateMutableCommandsKHR_t = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        mutable_config: *const cl_mutable_base_config_khr,
    ) -> cl_int,
>;

pub type clGetMutableCommandInfoKHR_t = Option<
    unsafe extern "C" fn(
        command: cl_mutable_command_khr,
        param_name: cl_mutable_command_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clSetMemObjectDestructorAPPLE_t = Option<
    unsafe extern "C" fn(
        memobj: cl_mem,
        pfn_notify: ::core::option::Option<
            unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void),
        >,
        user_data: *mut c_void,
    ) -> cl_int,
>;

pub type clLogMessagesToSystemLogAPPLE_t = Option<
    unsafe extern "C" fn(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    ),
>;

pub type clLogMessagesToStdoutAPPLE_t = Option<
    unsafe extern "C" fn(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    ),
>;

pub type clLogMessagesToStderrAPPLE_t = Option<
    unsafe extern "C" fn(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    ),
>;

pub type clIcdGetPlatformIDsKHR_t = Option<
    unsafe extern "C" fn(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int,
>;

pub type clCreateProgramWithILKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program,
>;

// cl_context_memory_initialize_khr
pub type cl_context_memory_initialize_khr = cl_bitfield;

pub type clTerminateContextKHR_t = Option<unsafe extern "C" fn(context: cl_context) -> cl_int>;

// cl_khr_create_command_queue extension
pub type cl_queue_properties_khr = cl_properties;

pub type clCreateCommandQueueWithPropertiesKHR_t = ::core::option::Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue,
>;

// cl_nv_device_attribute_query extension
pub type cl_nv_device_attribute_query = cl_uint;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cl_amd_device_topology {
    r#type: u32,
    unused: [u8; 17],
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

// cl_amd_device_attribute_query
pub type cl_amd_device_attribute_query = cl_uint;

// cl_device_partition_property_ext
pub type cl_device_partition_property_ext = cl_ulong;

pub type clReleaseDeviceEXT_t = Option<unsafe extern "C" fn(device: cl_device_id) -> cl_int>;

pub type clRetainDeviceEXT_t = Option<unsafe extern "C" fn(device: cl_device_id) -> cl_int>;

pub type clCreateSubDevicesEXT_t = Option<
    unsafe extern "C" fn(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property_ext,
        num_entries: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,
>;

// cl_ext_migrate_memobject extension definitions
pub type cl_mem_migration_flags_ext = cl_bitfield;

pub type clEnqueueMigrateMemObjectEXT_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags_ext,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// cl_qcom_ext_host_ptr extension
pub type cl_qcom_ext_host_ptr = cl_uint;
pub type cl_image_pitch_info_qcom = cl_uint;

pub type clGetDeviceImageInfoQCOM_t = Option<
    unsafe extern "C" fn(
        device: cl_device_id,
        image_width: size_t,
        image_height: size_t,
        image_format: *const cl_image_format,
        param_name: cl_image_pitch_info_qcom,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cl_mem_ext_host_ptr {
    pub allocation_type: cl_uint,
    pub host_cache_policy: cl_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mem_ion_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub ion_filedesc: cl_int,
    pub ion_hostptr: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mem_android_native_buffer_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub anb_ptr: *mut c_void,
}

pub type clEnqueueAcquireGrallocObjectsIMG_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseGrallocObjectsIMG_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// cl_img_generate_mipmap extension
pub type cl_mipmap_filter_mode_img = cl_uint;

pub type clEnqueueGenerateMipmapIMG_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        mipmap_filter_mode: cl_mipmap_filter_mode_img,
        array_region: *const size_t,
        mip_region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// To be used with the CL_MEM_ALLOC_FLAGS_IMG property
pub type cl_mem_alloc_flags_img = cl_bitfield;

pub type clGetKernelSubGroupInfoKHR_t = Option<
    unsafe extern "C" fn(
        in_kernel: cl_kernel,
        in_device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_khr_priority_hints extension
pub type cl_queue_priority_khr = cl_uint;

// cl_khr_throttle_hints extension
pub type cl_queue_throttle_khr = cl_uint;

// cl_khr_extended_versioning
pub type cl_version_khr = cl_uint;

// cl_khr_integer_dot_product
pub type cl_device_integer_dot_product_capabilities_khr = cl_bitfield;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_name_version_khr {
    pub version: cl_version_khr,
    pub name: [cl_uchar; CL_NAME_VERSION_MAX_NAME_SIZE_KHR],
}

// cl_khr_pci_bus_info
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cl_device_pci_bus_info_khr {
    pub pci_domain: cl_uint,
    pub pci_bus: cl_uint,
    pub pci_device: cl_uint,
    pub pci_function: cl_uint,
}

pub type clGetKernelSuggestedLocalWorkSizeKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        suggested_local_work_size: *mut size_t,
    ) -> cl_int,
>;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cl_device_integer_dot_product_acceleration_properties_khr {
    pub signed_accelerated: cl_bool,
    pub unsigned_accelerated: cl_bool,
    pub mixed_signedness_accelerated: cl_bool,
    pub accumulating_saturating_signed_accelerated: cl_bool,
    pub accumulating_saturating_unsigned_accelerated: cl_bool,
    pub accumulating_saturating_mixed_signedness_accelerated: cl_bool,
}

// cl_khr_external_memory
pub type cl_external_memory_handle_type_khr = cl_uint;

pub type clEnqueueAcquireExternalMemObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseExternalMemObjectsKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// cl_khr_external_semaphore
pub type cl_semaphore_khr = *mut c_void;
pub type cl_external_semaphore_handle_type_khr = cl_uint;
pub type cl_semaphore_properties_khr = cl_properties;

pub type clGetSemaphoreHandleForTypeKHR_t = Option<
    unsafe extern "C" fn(
        sema_object: cl_semaphore_khr,
        device: cl_device_id,
        handle_type: cl_external_semaphore_handle_type_khr,
        handle_size: size_t,
        handle_ptr: *mut c_void,
        handle_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_khr_external_semaphore_sync_fd

pub type cl_semaphore_reimport_properties_khr = cl_properties;

pub type clReImportSemaphoreSyncFdKHR_t = Option<
    unsafe extern "C" fn(
        sema_object: cl_semaphore_khr,
        reimport_props: *mut cl_semaphore_reimport_properties_khr,
        fd: c_int,
    ) -> cl_int,
>;

// cl_khr_semaphore

// pub type cl_semaphore_properties_khr = cl_properties; defined above
pub type cl_semaphore_info_khr = cl_uint;
pub type cl_semaphore_type_khr = cl_uint;
pub type cl_semaphore_payload_khr = cl_ulong;

pub type clCreateSemaphoreWithPropertiesKHR_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        sema_props: *const cl_semaphore_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_semaphore_khr,
>;

pub type clEnqueueWaitSemaphoresKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSignalSemaphoresKHR_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clGetSemaphoreInfoKHR_t = Option<
    unsafe extern "C" fn(
        sema_object: cl_semaphore_khr,
        param_name: cl_semaphore_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clReleaseSemaphoreKHR_t =
    Option<unsafe extern "C" fn(sema_object: cl_semaphore_khr) -> cl_int>;

pub type clRetainSemaphoreKHR_t =
    Option<unsafe extern "C" fn(sema_object: cl_semaphore_khr) -> cl_int>;

// cl_arm_import_memory extension

pub type cl_import_properties_arm = intptr_t;

pub type clImportMemoryARM_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        properties: *const cl_import_properties_arm,
        memory: *mut c_void,
        size: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

// cl_arm_shared_virtual_memory extension

// Used by clSetKernelExecInfoARM:
pub type cl_kernel_exec_info_arm = cl_uint;

// Flag values returned by clGetDeviceInfo with CL_DEVICE_SVM_CAPABILITIES_ARM as the param_name.
pub type cl_device_svm_capabilities_arm = cl_bitfield;

// Flag values used by clSVMAllocARM:
pub type cl_svm_mem_flags_arm = cl_bitfield;

pub type clSVMAllocARM_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        flags: cl_svm_mem_flags_arm,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void,
>;

pub type clSVMFreeARM_t =
    Option<unsafe extern "C" fn(context: cl_context, svm_pointer: *mut c_void)>;

pub type clEnqueueSVMFreeARM_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *mut *mut c_void,
        pfn_free_func: Option<
            unsafe extern "C" fn(
                queue: cl_command_queue,
                num_svm_pointers: cl_uint,
                svm_pointers: *mut *mut c_void,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSVMMemcpyARM_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSVMMemFillARM_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSVMMapARM_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSVMUnmapARM_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clSetKernelArgSVMPointerARM_t = Option<
    unsafe extern "C" fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int,
>;

pub type clSetKernelExecInfoARM_t = Option<
    unsafe extern "C" fn(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info_arm,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int,
>;

// cl_arm_scheduling_controls
pub type cl_device_scheduling_controls_capabilities_arm = cl_bitfield;

// cl_arm_controlled_kernel_termination

// Bit fields for controlled termination feature query
pub type cl_device_controlled_termination_capabilities_arm = cl_bitfield;

// Values returned for event termination reason query
pub type cl_command_termination_reason_arm = cl_uint;

// cl_intel_device_attribute_query
pub type cl_device_feature_capabilities_intel = cl_bitfield;

// cl_intel_accelerator extension
// cl_intel_motion_estimation extension
// cl_intel_advanced_motion_estimation extension

pub type cl_accelerator_intel = *mut c_void;
pub type cl_accelerator_type_intel = cl_uint;
pub type cl_accelerator_info_intel = cl_uint;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cl_motion_estimation_desc_intel {
    pub mb_block_type: cl_uint,
    pub subpixel_mode: cl_uint,
    pub sad_adjust_mode: cl_uint,
    pub search_path_type: cl_uint,
}

// cl_motion_detect_desc_intel flags
pub type cl_motion_detect_desc_intel = cl_uint;

pub type clCreateAcceleratorINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        accelerator_type: cl_accelerator_type_intel,
        descriptor_size: size_t,
        descriptor: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_accelerator_intel,
>;

pub type clGetAcceleratorInfoINTEL_t = Option<
    unsafe extern "C" fn(
        accelerator: cl_accelerator_intel,
        param_name: cl_accelerator_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clRetainAcceleratorINTEL_t =
    Option<unsafe extern "C" fn(accelerator: cl_accelerator_intel) -> cl_int>;

pub type clReleaseAcceleratorINTEL_t =
    Option<unsafe extern "C" fn(accelerator: cl_accelerator_intel) -> cl_int>;

// cl_intel_driver_diagnostics extension
pub type cl_diagnostics_verbose_level = cl_uint;

// cl_intel_device_side_avc_motion_estimation extension
pub type cl_intel_avc_motion_estimation = cl_uint;

// cl_intel_unified_shared_memory extension
pub type cl_device_unified_shared_memory_capabilities_intel = cl_bitfield;
pub type cl_mem_properties_intel = cl_properties;
pub type cl_mem_alloc_flags_intel = cl_bitfield;
pub type cl_mem_info_intel = cl_uint;
pub type cl_unified_shared_memory_type_intel = cl_uint;
pub type cl_mem_advice_intel = cl_uint;

pub type clHostMemAllocINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clDeviceMemAllocINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clSharedMemAllocINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clMemFreeINTEL_t =
    Option<unsafe extern "C" fn(context: cl_context, ptr: *mut c_void) -> cl_int>;

pub type clMemBlockingFreeINTEL_t =
    Option<unsafe extern "C" fn(context: cl_context, ptr: *mut c_void) -> cl_int>;

pub type clGetMemAllocInfoINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        ptr: *const c_void,
        param_name: cl_mem_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clSetKernelArgMemPointerINTEL_t = Option<
    unsafe extern "C" fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int,
>;

pub type clEnqueueMemFillINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemcpyINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        blocking: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemAdviseINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        advice: cl_mem_advice_intel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMigrateMemINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemsetINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        value: cl_int,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clCreateBufferWithPropertiesINTEL_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

pub type clEnqueueReadHostPipeINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        program: cl_program,
        pipe_symbol: *const c_char,
        blocking_read: cl_bool,
        ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueWriteHostPipeINTEL_t = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        program: cl_program,
        pipe_symbol: *const c_char,
        blocking_write: cl_bool,
        ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type cl_command_queue_capabilities_intel = cl_bitfield;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_queue_family_properties_intel {
    pub properties: cl_command_queue_properties,
    pub capabilities: cl_command_queue_capabilities_intel,
    pub count: cl_uint,
    pub name: [cl_uchar; CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL],
}

// cl_ext_image_requirements_info

pub type cl_image_requirements_info_ext = cl_uint;

pub type clGetImageRequirementsInfoEXT_t = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        param_name: cl_image_requirements_info_ext,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_loader_info

pub type cl_icdl_info = cl_uint;

pub type clGetICDLoaderInfoOCLICD_t = Option<
    unsafe extern "C" fn(
        param_name: cl_icdl_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_pocl_content_size

pub type cl_device_fp_atomic_capabilities_ext = cl_bitfield;

pub type clSetContentSizeBufferPoCL_t =
    Option<unsafe extern "C" fn(buffer: cl_mem, content_size_buffer: cl_mem) -> cl_int>;
