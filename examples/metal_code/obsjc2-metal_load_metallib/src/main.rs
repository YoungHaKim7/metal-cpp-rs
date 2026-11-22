use objc2_foundation::{NSURL, ns_string};
use objc2_metal::MTLComputePipelineState;
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLResourceOptions, MTLSize,
};
use std::ptr::NonNull;

fn main() {
    // 1. Get the default Metal GPU device
    let device = MTLCreateSystemDefaultDevice().expect("No Metal device found");

    // 2. Load the .metallib file from disk via NSURL
    let url = NSURL::fileURLWithPath(ns_string!("compute.metallib"));
    let library = device
        .newLibraryWithURL_error(&url)
        .expect("Failed to load compute.metallib");

    // 3. Look up the kernel function by name and build a pipeline state
    let function = library
        .newFunctionWithName(ns_string!("double_values"))
        .expect("Function 'double_values' not found in metallib");

    let pipeline_state = device
        .newComputePipelineStateWithFunction_error(&function)
        .expect("Failed to create compute pipeline state");

    // 4. Allocate a shared GPU buffer and populate it with input data
    let input_data: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let buffer_size = std::mem::size_of_val(&input_data);

    let buffer = unsafe {
        let ptr = NonNull::new(input_data.as_ptr() as *mut _).unwrap();
        device
            .newBufferWithBytes_length_options(
                ptr,
                buffer_size,
                MTLResourceOptions::StorageModeShared,
            )
            .expect("Failed to allocate buffer")
    };

    // 5. Create a command queue, command buffer, and compute encoder
    let command_queue = device
        .newCommandQueue()
        .expect("Failed to create command queue");
    let command_buffer = command_queue
        .commandBuffer()
        .expect("Failed to create command buffer");
    let encoder = command_buffer
        .computeCommandEncoder()
        .expect("Failed to create compute command encoder");

    // 6. Bind pipeline and buffer, then dispatch threads
    encoder.setComputePipelineState(&pipeline_state);
    unsafe {
        encoder.setBuffer_offset_atIndex(Some(&buffer), 0, 0);
    }

    let grid_size = MTLSize {
        width: input_data.len(),
        height: 1,
        depth: 1,
    };
    let threadgroup_size = MTLSize {
        width: pipeline_state
            .maxTotalThreadsPerThreadgroup()
            .min(input_data.len()),
        height: 1,
        depth: 1,
    };

    encoder.dispatchThreads_threadsPerThreadgroup(grid_size, threadgroup_size);
    encoder.endEncoding();

    // 7. Execute and wait for the GPU to finish
    command_buffer.commit();
    command_buffer.waitUntilCompleted();

    // 8. Read back results from the shared buffer
    let result_ptr = buffer.contents().as_ptr() as *const f32;
    let results = unsafe { std::slice::from_raw_parts(result_ptr, input_data.len()) };
    println!("Output: {:?}", results); // [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]
}
