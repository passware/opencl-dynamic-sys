#![allow(non_snake_case)]

use dlopen2::wrapper::WrapperApi;

use crate::types::{cl_dx9_media_sharing::*, cl_egl::*, cl_gl::*, cl_platform::*, *};

mod utils;
pub use utils::{load_library, OpenClRuntime};

#[derive(WrapperApi)]
pub struct OpenCl {
    // Platform API
    clGetPlatformIDs: fn(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int,

    clGetPlatformInfo: fn(
        platform: cl_platform_id,
        param_name: cl_platform_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Device APIs
    clGetDeviceIDs: fn(
        platform: cl_platform_id,
        device_type: cl_device_type,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,

    clGetDeviceInfo: fn(
        device: cl_device_id,
        param_name: cl_device_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clCreateSubDevices: fn(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property,
        num_devices: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices_ret: *mut cl_uint,
    ) -> cl_int,

    clRetainDevice: fn(device: cl_device_id) -> cl_int,

    clReleaseDevice: fn(device: cl_device_id) -> cl_int,

    clSetDefaultDeviceCommandQueue:
        fn(context: cl_context, device: cl_device_id, command_queue: cl_command_queue) -> cl_int,

    clGetDeviceAndHostTimer: fn(
        device: cl_device_id,
        device_timestamp: *mut cl_ulong,
        host_timestamp: *mut cl_ulong,
    ) -> cl_int,

    clGetHostTimer: fn(device: cl_device_id, host_timestamp: *mut cl_ulong) -> cl_int,

    // Context APIs
    clCreateContext: fn(
        properties: *const cl_context_properties,
        num_devices: cl_uint,
        devices: *const cl_device_id,
        pfn_notify: Option<
            unsafe extern "C" fn(
                errinfo: *const c_char,
                private_info: *const c_void,
                cb: size_t,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context,

    clCreateContextFromType: fn(
        properties: *const cl_context_properties,
        device_type: cl_device_type,
        pfn_notify: Option<
            unsafe extern "C" fn(
                errinfo: *const c_char,
                private_info: *const c_void,
                cb: size_t,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context,

    clRetainContext: fn(context: cl_context) -> cl_int,

    clReleaseContext: fn(context: cl_context) -> cl_int,

    clGetContextInfo: fn(
        context: cl_context,
        param_name: cl_context_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clSetContextDestructorCallback: fn(
        context: cl_context,
        pfn_notify: Option<unsafe extern "C" fn(context: cl_context, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int,

    // Command Queue APIs
    clCreateCommandQueueWithProperties: fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue,

    clRetainCommandQueue: fn(command_queue: cl_command_queue) -> cl_int,

    clReleaseCommandQueue: fn(command_queue: cl_command_queue) -> cl_int,

    clGetCommandQueueInfo: fn(
        command_queue: cl_command_queue,
        param_name: cl_command_queue_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Memory Object APIs
    clCreateBuffer: fn(
        context: cl_context,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateSubBuffer: fn(
        buffer: cl_mem,
        flags: cl_mem_flags,
        buffer_create_type: cl_buffer_create_type,
        buffer_create_info: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateImage: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreatePipe: fn(
        context: cl_context,
        flags: cl_mem_flags,
        pipe_packet_size: cl_uint,
        pipe_max_packets: cl_uint,
        properties: *const cl_pipe_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateBufferWithProperties: fn(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateImageWithProperties: fn(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clRetainMemObject: fn(memobj: cl_mem) -> cl_int,

    clReleaseMemObject: fn(memobj: cl_mem) -> cl_int,

    clGetSupportedImageFormats: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        image_formats: *mut cl_image_format,
        num_image_formats: *mut cl_uint,
    ) -> cl_int,

    clGetMemObjectInfo: fn(
        memobj: cl_mem,
        param_name: cl_mem_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetImageInfo: fn(
        image: cl_mem,
        param_name: cl_image_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetPipeInfo: fn(
        pipe: cl_mem,
        param_name: cl_pipe_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clSetMemObjectDestructorCallback: fn(
        memobj: cl_mem,
        pfn_notify: Option<unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int,

    // SVM Allocation APIs
    clSVMAlloc: fn(
        context: cl_context,
        flags: cl_svm_mem_flags,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void,

    clSVMFree: fn(context: cl_context, svm_pointer: *mut c_void),

    // Sampler APIs
    clCreateSamplerWithProperties: fn(
        context: cl_context,
        normalized_coords: *const cl_sampler_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler,

    clRetainSampler: fn(sampler: cl_sampler) -> cl_int,

    clReleaseSampler: fn(sampler: cl_sampler) -> cl_int,

    clGetSamplerInfo: fn(
        sampler: cl_sampler,
        param_name: cl_sampler_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Program Object APIs
    clCreateProgramWithSource: fn(
        context: cl_context,
        count: cl_uint,
        strings: *const *const c_char,
        lengths: *const size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program,

    clCreateProgramWithBinary: fn(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        lengths: *const size_t,
        binaries: *const *const c_uchar,
        binary_status: *mut cl_int,
        errcode_ret: *mut cl_int,
    ) -> cl_program,

    clCreateProgramWithBuiltInKernels: fn(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        kernel_names: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_program,

    clCreateProgramWithIL: fn(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program,

    clRetainProgram: fn(program: cl_program) -> cl_int,

    clReleaseProgram: fn(program: cl_program) -> cl_int,

    clBuildProgram: fn(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int,

    clCompileProgram: fn(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_headers: cl_uint,
        input_headers: *const cl_program,
        header_include_names: *const *const c_char,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int,

    clLinkProgram: fn(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_programs: cl_uint,
        input_programs: *const cl_program,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_program,

    clSetProgramReleaseCallback: fn(
        program: cl_program,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int,

    clSetProgramSpecializationConstant: fn(
        program: cl_program,
        spec_id: cl_uint,
        spec_size: size_t,
        spec_value: *const c_void,
    ) -> cl_int,

    clUnloadPlatformCompiler: fn(platform: cl_platform_id) -> cl_int,

    clGetProgramInfo: fn(
        program: cl_program,
        param_name: cl_program_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetProgramBuildInfo: fn(
        program: cl_program,
        device: cl_device_id,
        param_name: cl_program_build_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Kernel Object APIs
    clCreateKernel:
        fn(program: cl_program, kernel_name: *const c_char, errcode_ret: *mut cl_int) -> cl_kernel,

    clCreateKernelsInProgram: fn(
        program: cl_program,
        num_kernels: cl_uint,
        kernels: *mut cl_kernel,
        num_kernels_ret: *mut cl_uint,
    ) -> cl_int,

    clCloneKernel: fn(source_kernel: cl_kernel, errcode_ret: *mut cl_int) -> cl_kernel,

    clRetainKernel: fn(kernel: cl_kernel) -> cl_int,

    clReleaseKernel: fn(kernel: cl_kernel) -> cl_int,

    clSetKernelArg: fn(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_size: size_t,
        arg_value: *const c_void,
    ) -> cl_int,

    clSetKernelArgSVMPointer:
        fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int,

    clSetKernelExecInfo: fn(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int,

    clGetKernelInfo: fn(
        kernel: cl_kernel,
        param_name: cl_kernel_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetKernelArgInfo: fn(
        kernel: cl_kernel,
        arg_indx: cl_uint,
        param_name: cl_kernel_arg_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetKernelWorkGroupInfo: fn(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_work_group_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clGetKernelSubGroupInfo: fn(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Event Object APIs
    clWaitForEvents: fn(num_events: cl_uint, event_list: *const cl_event) -> cl_int,

    clGetEventInfo: fn(
        event: cl_event,
        param_name: cl_event_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clCreateUserEvent: fn(context: cl_context, errcode_ret: *mut cl_int) -> cl_event,

    clRetainEvent: fn(event: cl_event) -> cl_int,

    clReleaseEvent: fn(event: cl_event) -> cl_int,

    clSetUserEventStatus: fn(event: cl_event, execution_status: cl_int) -> cl_int,

    clSetEventCallback: fn(
        event: cl_event,
        command_exec_callback_type: cl_int,
        pfn_notify: Option<
            unsafe extern "C" fn(
                event: cl_event,
                event_command_status: cl_int,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
    ) -> cl_int,

    // Profiling APIs
    clGetEventProfilingInfo: fn(
        event: cl_event,
        param_name: cl_profiling_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    // Flush and Finish APIs
    clFlush: fn(command_queue: cl_command_queue) -> cl_int,

    clFinish: fn(command_queue: cl_command_queue) -> cl_int,

    // Enqueued Commands APIs
    clEnqueueReadBuffer: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        offset: size_t,
        cb: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueReadBufferRect: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slc_pitch: size_t,
        host_row_pitch: size_t,
        host_slc_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueWriteBuffer: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        offset: size_t,
        cb: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueWriteBufferRect: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slc_pitch: size_t,
        host_row_pitch: size_t,
        host_slc_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueFillBuffer: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueCopyBuffer: fn(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        cb: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueCopyBufferRect: fn(
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
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueReadImage: fn(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_read: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        row_pitch: size_t,
        slice_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueWriteImage: fn(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_write: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        input_row_pitch: size_t,
        input_slice_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueFillImage: fn(
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueCopyImage: fn(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueCopyImageToBuffer: fn(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueCopyBufferToImage: fn(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueMapBuffer: fn(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,

    clEnqueueMapImage: fn(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        origin: *const size_t,
        region: *const size_t,
        image_row_pitch: *mut size_t,
        image_slice_pitch: *mut size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,

    clEnqueueUnmapMemObject: fn(
        command_queue: cl_command_queue,
        memobj: cl_mem,
        mapped_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueMigrateMemObjects: fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueNDRangeKernel: fn(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_dims: *const size_t,
        local_work_dims: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueNativeKernel: fn(
        command_queue: cl_command_queue,
        user_func: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        args: *mut c_void,
        cb_args: size_t,
        num_mem_objects: cl_uint,
        mem_list: *const cl_mem,
        args_mem_loc: *const *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueMarkerWithWaitList: fn(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueBarrierWithWaitList: fn(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueSVMFree: fn(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
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

    clEnqueueSVMMemcpy: fn(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueSVMMemFill: fn(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueSVMMap: fn(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueSVMUnmap: fn(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueSVMMigrateMem: fn(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
        sizes: *const size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clGetExtensionFunctionAddressForPlatform:
        fn(platform: cl_platform_id, func_name: *const c_char) -> *mut c_void,

    // Deprecated OpenCL 1.1 APIs
    clCreateImage2D: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_depth: size_t,
        image_row_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateImage3D: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_height: size_t,
        image_depth: size_t,
        image_row_pitch: size_t,
        image_slice_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clEnqueueMarker: fn(command_queue: cl_command_queue, event: *mut cl_event) -> cl_int,

    clEnqueueWaitForEvents: fn(
        command_queue: cl_command_queue,
        num_events: cl_uint,
        event_list: *mut cl_event,
    ) -> cl_int,

    clEnqueueBarrier: fn(command_queue: cl_command_queue) -> cl_int,

    clUnloadCompiler: fn() -> cl_int,

    clGetExtensionFunctionAddress: fn(func_name: *const c_char),

    // Deprecated OpenCL 2.0 APIs
    clCreateCommandQueue: fn(
        context: cl_context,
        device: cl_device_id,
        properties: cl_command_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue,

    clCreateSampler: fn(
        context: cl_context,
        normalize_coords: cl_bool,
        addressing_mode: cl_addressing_mode,
        filter_mode: cl_filter_mode,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler,

    // Deprecated 1.2
    clEnqueueTask: fn(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    // Direct3D 10 APIs
    clGetSupportedD3D10TextureFormatsINTEL: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        d3d10_formats: *mut DXGI_FORMAT,
        num_surface_formats: *mut cl_uint,
    ) -> cl_int,

    // Direct3D 11 APIs
    clGetSupportedD3D11TextureFormatsINTEL: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        plane: cl_uint,
        num_entries: cl_uint,
        d3d11_formats: *mut DXGI_FORMAT,
        num_surface_formats: *mut cl_uint,
    ) -> cl_int,

    // DirectX9 Media Sharing APIs
    clGetDeviceIDsFromDX9INTEL: fn(
        platform: cl_platform_id,
        dx9_device_source: cl_dx9_device_source_intel,
        dx9_object: *mut c_void,
        dx9_device_set: cl_dx9_device_set_intel,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,

    clCreateFromDX9MediaSurfaceINTEL: fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: IDirect3DSurface9_ptr,
        sharedHandle: HANDLE,
        plane: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clEnqueueAcquireDX9ObjectsINTEL: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueReleaseDX9ObjectsINTEL: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clGetSupportedDX9MediaSurfaceFormatsINTEL: fn(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        plane: cl_uint,
        num_entries: cl_uint,
        dx9_formats: *mut D3DFORMAT,
        num_surface_formats: *mut cl_uint,
    ) -> cl_int,

    // EGL APIs
    clCreateFromEGLImageKHR: fn(
        context: cl_context,
        display: CLeglDisplayKHR,
        image: CLeglImageKHR,
        flags: cl_mem_flags,
        properties: *const cl_egl_image_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clEnqueueAcquireEGLObjectsKHR: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueReleaseEGLObjectsKHR: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clCreateEventFromEGLSyncKHR: fn(
        context: cl_context,
        sync: CLeglSyncKHR,
        display: CLeglDisplayKHR,
        errcode_ret: *mut cl_int,
    ) -> cl_event,

    // OpenGL APIs
    clCreateFromGLBuffer: fn(
        context: cl_context,
        flags: cl_mem_flags,
        bufobj: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateFromGLTexture: fn(
        context: cl_context,
        flags: cl_mem_flags,
        target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateFromGLRenderbuffer: fn(
        context: cl_context,
        flags: cl_mem_flags,
        renderbuffer: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clGetGLObjectInfo: fn(
        memobj: cl_mem,
        gl_object_type: *mut cl_gl_object_type,
        gl_object_name: *mut cl_GLuint,
    ) -> cl_int,

    clGetGLTextureInfo: fn(
        memobj: cl_mem,
        param_name: cl_gl_texture_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,

    clEnqueueAcquireGLObjects: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    clEnqueueReleaseGLObjects: fn(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,

    // Deprecated OpenCL 1.1 APIs
    clCreateFromGLTexture2D: fn(
        context: cl_context,
        flags: cl_mem_flags,
        texture_target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,

    clCreateFromGLTexture3D: fn(
        context: cl_context,
        flags: cl_mem_flags,
        texture_target: cl_GLenum,
        miplevel: cl_GLint,
        texture: cl_GLuint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
}
