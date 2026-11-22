//! offscreen_triangle — renders a gradient triangle into an offscreen
//! texture and dumps it as a PPM. Runtime-compiles the Metal shader source
//! (the same flow as Apple's "Hello, Triangle!" sample), then:
//!
//! device → library(source) → render pipeline → command buffer →
//! render command encoder → draw → commit → read back.
//!
//! This is the FFI equivalent of the metal-cpp program in Apple's sample;
//! every call goes through the exact selector metal-cpp's bridge uses.

use metal_cpp_rs::objc_abi::autoreleasepool;
use metal_cpp_rs::prelude::*;

const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut triangle_vertex(uint vid [[vertex_id]]) {
    float2 positions[3] = {
        float2(-0.5, -0.5), float2(0.5, -0.5), float2(0.0, 0.5)
    };
    float3 colors[3] = {
        float3(1.0, 0.2, 0.2), float3(0.2, 1.0, 0.2), float3(0.2, 0.2, 1.0)
    };
    VertexOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = float4(colors[vid], 1.0);
    return out;
}

fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

const WIDTH: u64 = 256;
const HEIGHT: u64 = 256;

fn main() {
    autoreleasepool(|| unsafe {
        // 1. Device.
        let device = MTLCreateSystemDefaultDevice();
        if device.is_nil() {
            eprintln!("MTLCreateSystemDefaultDevice() returned nil");
            std::process::exit(1);
        }
        println!("device: {}", device.name().to_string());

        // 2. Compile shader source at runtime.
        let source = NSString::from_str(SHADER_SOURCE);
        let mut error = Strong::<NSError>::nil();
        let compile_options = MTLCompileOptions::new();
        let library = device.newLibrary(source.as_ref(), compile_options.as_ref(), &mut error);
        if library.is_nil() {
            eprintln!(
                "shader compile failed: {}",
                if error.is_nil() {
                    "<no error object>".to_string()
                } else {
                    error.localizedDescription().to_string()
                }
            );
            std::process::exit(1);
        }

        let vertex_fn = library.newFunction(NSString::from_str("triangle_vertex").as_ref());
        let fragment_fn = library.newFunction(NSString::from_str("triangle_fragment").as_ref());
        if vertex_fn.is_nil() || fragment_fn.is_nil() {
            eprintln!("function lookup failed");
            std::process::exit(1);
        }

        // 3. Render pipeline.
        let pipeline_descriptor = MTLRenderPipelineDescriptor::new();
        pipeline_descriptor.setVertexFunction(vertex_fn.as_ref());
        pipeline_descriptor.setFragmentFunction(fragment_fn.as_ref());
        pipeline_descriptor
            .colorAttachments()
            .objectAtIndexedSubscript(0)
            .setPixelFormat(MTLPixelFormat::PixelFormatBGRA8Unorm);

        let mut error = Strong::<NSError>::nil();
        let pipeline = device.newRenderPipelineState(pipeline_descriptor.as_ref(), &mut error);
        if pipeline.is_nil() {
            eprintln!(
                "pipeline creation failed: {}",
                if error.is_nil() {
                    "<no error object>".to_string()
                } else {
                    error.localizedDescription().to_string()
                }
            );
            std::process::exit(1);
        }

        // 4. Offscreen render target.
        let texture_descriptor = MTLTextureDescriptor::texture2DDescriptor(
            MTLPixelFormat::PixelFormatBGRA8Unorm,
            WIDTH,
            HEIGHT,
            false,
        );
        texture_descriptor.setUsage(
            MTLTextureUsage::TextureUsageRenderTarget | MTLTextureUsage::TextureUsageShaderRead,
        );
        texture_descriptor.setStorageMode(MTLStorageMode::StorageModeManaged);
        let texture = device.newTexture(texture_descriptor.as_ref());

        // 5. Render pass.
        let render_pass = MTLRenderPassDescriptor::renderPassDescriptor();
        let color0 = render_pass.colorAttachments().objectAtIndexedSubscript(0);
        let attachment = color0.as_attachment();
        attachment.setTexture(texture.as_ref());
        attachment.setLoadAction(MTLLoadAction::LoadActionClear);
        attachment.setStoreAction(MTLStoreAction::StoreActionStore);
        color0.setClearColor(MTLClearColor::Make(0.05, 0.05, 0.08, 1.0));

        // 6. Encode + submit.
        let queue = device.newCommandQueue();
        let command_buffer = queue.commandBuffer();
        let encoder = command_buffer.renderCommandEncoder(render_pass.as_ref());
        encoder.setRenderPipelineState(pipeline.as_ref());
        encoder.drawPrimitives(MTLPrimitiveType::PrimitiveTypeTriangle, 0, 3);
        encoder.endEncoding();
        command_buffer.commit();
        command_buffer.waitUntilCompleted();

        if command_buffer.status() == MTLCommandBufferStatus::CommandBufferStatusError {
            eprintln!("command buffer errored: {}", {
                let e = command_buffer.error();
                if e.is_nil() {
                    "<no error>".to_string()
                } else {
                    e.localizedDescription().to_string()
                }
            });
            std::process::exit(1);
        }

        // 7. Read back and write PPM (managed storage: blit to shared buffer).
        let bytes_per_row = WIDTH * 4;
        let shared_buffer = device.newBuffer(
            bytes_per_row * HEIGHT,
            MTLResourceOptions::ResourceStorageModeShared,
        );
        command_buffer_after_(&queue, shared_buffer.as_ref(), texture.as_ref());

        let src = shared_buffer.contents();
        std::fs::write(
            "triangle.ppm",
            build_ppm(
                src as *const u8,
                bytes_per_row as usize,
                WIDTH as usize,
                HEIGHT as usize,
            ),
        )
        .expect("write triangle.ppm");
        println!("wrote triangle.ppm ({}x{})", WIDTH, HEIGHT);
    });
}

/// Second command buffer that blits the (managed) render target into the
/// shared buffer for CPU read-back.
unsafe fn command_buffer_after_(
    queue: &Strong<MTLCommandQueue>,
    shared_buffer: Ref<MTLBuffer>,
    texture: Ref<MTLTexture>,
) {
    unsafe {
        let cb = queue.commandBuffer();
        let blit = cb.blitCommandEncoder();
        blit.copyFromTextureToBuffer(
            texture,
            0, // sourceSlice
            0, // sourceLevel
            MTLOrigin::Make(0, 0, 0),
            MTLSize::Make(WIDTH, HEIGHT, 1),
            shared_buffer,
            0,                  // destinationOffset
            WIDTH * 4,          // destinationBytesPerRow
            WIDTH * 4 * HEIGHT, // destinationBytesPerImage
        );
        blit.endEncoding();
        cb.commit();
        cb.waitUntilCompleted();
    }
}

/// Minimal binary PPM encoder from the BGRA readback buffer.
fn build_ppm(bgra: *const u8, bytes_per_row: usize, w: usize, h: usize) -> Vec<u8> {
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for y in 0..h {
        for x in 0..w {
            unsafe {
                let p = bgra.add(y * bytes_per_row + x * 4);
                out.push(*p.add(2)); // R
                out.push(*p.add(1)); // G
                out.push(*p.add(0)); // B
            }
        }
    }
    out
}
