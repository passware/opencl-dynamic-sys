use crate::types::{c_void, cl_bitfield, cl_properties, cl_uint, cl_ulong, intptr_t};

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

// cl_khr_command_buffer_mutable_dispatch
pub type cl_command_buffer_structure_type_khr = cl_uint;
pub type cl_mutable_dispatch_fields_khr = cl_bitfield;
pub type cl_mutable_command_info_khr = cl_uint;
pub type cl_mutable_dispatch_asserts_khr = cl_bitfield;

// cl_context_memory_initialize_khr
pub type cl_context_memory_initialize_khr = cl_bitfield;

// cl_khr_create_command_queue extension
pub type cl_queue_properties_khr = cl_properties;

// cl_nv_device_attribute_query extension
pub type cl_nv_device_attribute_query = cl_uint;

// cl_amd_device_attribute_query
pub type cl_amd_device_attribute_query = cl_uint;

// cl_device_partition_property_ext
pub type cl_device_partition_property_ext = cl_ulong;

// cl_ext_migrate_memobject extension definitions
pub type cl_mem_migration_flags_ext = cl_bitfield;

// cl_qcom_ext_host_ptr extension
pub type cl_qcom_ext_host_ptr = cl_uint;
pub type cl_image_pitch_info_qcom = cl_uint;

// cl_img_generate_mipmap extension
pub type cl_mipmap_filter_mode_img = cl_uint;

// To be used with the CL_MEM_ALLOC_FLAGS_IMG property
pub type cl_mem_alloc_flags_img = cl_bitfield;

// cl_khr_priority_hints extension
pub type cl_queue_priority_khr = cl_uint;

// cl_khr_throttle_hints extension
pub type cl_queue_throttle_khr = cl_uint;

// cl_khr_extended_versioning
pub type cl_version_khr = cl_uint;

// cl_khr_integer_dot_product
pub type cl_device_integer_dot_product_capabilities_khr = cl_bitfield;

// cl_khr_external_memory
pub type cl_external_memory_handle_type_khr = cl_uint;

// cl_khr_external_semaphore
pub type cl_semaphore_khr = *mut c_void;
pub type cl_external_semaphore_handle_type_khr = cl_uint;
pub type cl_semaphore_properties_khr = cl_properties;

// cl_khr_external_semaphore_sync_fd
pub type cl_semaphore_reimport_properties_khr = cl_properties;

// cl_khr_semaphore
pub type cl_semaphore_info_khr = cl_uint;
pub type cl_semaphore_type_khr = cl_uint;
pub type cl_semaphore_payload_khr = cl_ulong;

// cl_arm_import_memory extension
pub type cl_import_properties_arm = intptr_t;

// Used by clSetKernelExecInfoARM:
pub type cl_kernel_exec_info_arm = cl_uint;

// Flag values returned by clGetDeviceInfo with CL_DEVICE_SVM_CAPABILITIES_ARM as the param_name.
pub type cl_device_svm_capabilities_arm = cl_bitfield;

// Flag values used by clSVMAllocARM:
pub type cl_svm_mem_flags_arm = cl_bitfield;

// cl_device_info
pub type cl_device_scheduling_controls_capabilities_arm = cl_bitfield;

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

// cl_motion_detect_desc_intel flags
pub type cl_motion_detect_desc_intel = cl_uint;

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

// cl_intel_command_queue_families
pub type cl_command_queue_capabilities_intel = cl_bitfield;

// cl_ext_image_requirements_info
pub type cl_image_requirements_info_ext = cl_uint;

// cl_loader_info
pub type cl_icdl_info = cl_uint;

// cl_pocl_content_size
pub type cl_device_fp_atomic_capabilities_ext = cl_bitfield;
