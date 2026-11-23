//! # 10 — Frame Debugging: Capture GPU Commands for Debugging
//!
//! Uses `MTLCaptureManager` to capture a single frame's GPU commands into
//! a `.gputrace` file that you can open in Xcode's GPU debugger. Capturing
//! one frame is enough to inspect every draw call, shader, and resource.
//!
//! The sample renders a spinning triangle (reusing sample 03's animation)
//! and captures the first ~60 frames to `./frame_debugging.gputrace`.
//!
//! Run with: `cargo run -p frame_debugging`
//!
//! After running, open the trace in Xcode:
//! ```sh
//! open frame_debugging.gputrace
//! ```

use metal_cpp_rs::objc_abi::{Ref, Sel};
use metal_cpp_rs::prelude::*;
use shared::{
    App, Frame, MTLClearColor, MTLCompileOptions, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType,
    MTLRenderPassDescriptor, MTLRenderPipelineDescriptor, MTLResourceOptions, MTLStoreAction,
};

const SHADER_SOURCE: &str = include_str!("../../assets/frame_debugging.metal");

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.2, 0.2, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.2, 1.0, 0.2, 1.0],
    },
    Vertex {
        position: [0.0, 0.5],
        color: [0.2, 0.2, 1.0, 1.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct Args {
    rotation: f32,
}

fn main() {
    let app = App::new("10 - Frame Debugging", 800.0, 600.0);
    let device = app.device().as_ref();
    let pipeline = unsafe { build_pipeline(device, SHADER_SOURCE, "vs", "fs") };
    let vb = unsafe { upload(device, &VERTICES) };
    let ub = unsafe { device.newBuffer(4, MTLResourceOptions::ResourceStorageModeShared) };

    // --- Set up MTLCaptureManager ---
    // Capture the first 60 frames to a file, then stop.
    let capture_path = std::ffi::CString::new("frame_debugging.gputrace").unwrap();
    let mut frames_captured: u32 = 0;
    let capture_started = unsafe { start_capture(device, capture_path.as_ptr()) };
    if capture_started {
        println!("GPU capture started → will write to ./frame_debugging.gputrace");
    }

    app.run(|frame: &Frame| unsafe {
        let args = Args {
            rotation: frame.time,
        };
        *(ub.contents() as *mut Args) = args;

        let tex: Ref<MTLTexture> =
            msg_send!(frame.drawable, Sel::get("texture\0").as_ptr().unwrap());
        let rp = MTLRenderPassDescriptor::renderPassDescriptor();
        let ca = rp.colorAttachments().objectAtIndexedSubscript(0);
        ca.as_attachment().setTexture(tex);
        ca.as_attachment()
            .setLoadAction(MTLLoadAction::LoadActionClear);
        ca.as_attachment()
            .setStoreAction(MTLStoreAction::StoreActionStore);
        ca.setClearColor(MTLClearColor::Make(0.05, 0.05, 0.08, 1.0));

        let cb = frame.queue.commandBuffer();
        let enc = cb.renderCommandEncoder(rp.as_ref());
        enc.setRenderPipelineState(pipeline.as_ref());
        enc.setVertexBuffer(vb.as_ref(), 0, 0);
        enc.setVertexBuffer(ub.as_ref(), 0, 1);
        enc.drawPrimitives(MTLPrimitiveType::PrimitiveTypeTriangle, 0, 3);
        enc.endEncoding();

        let _: () = msg_send!(
            cb,
            Sel::get("presentDrawable:\0").as_ptr().unwrap(),
            frame.drawable
        );
        cb.commit();
        cb.waitUntilCompleted();

        frames_captured += 1;
        if capture_started && frames_captured >= 60 {
            println!("Captured 60 frames. Stopping capture.");
            stop_capture();
            println!("Open the trace in Xcode: open frame_debugging.gputrace");
        }
    });
}

/// Start a GPU capture for the given device, writing to `path`.
///
/// Uses `MTLCaptureManager.sharedCaptureManager` +
/// `MTLCaptureDescriptor` with `MTLCaptureDestination::CaptureDestinationGPUTraceDocument`.
unsafe fn start_capture(device: Ref<MTLDevice>, path: *const std::ffi::c_char) -> bool {
    let mgr_sel = Sel::get("sharedCaptureManager\0").as_ptr().unwrap();
    let mgr: Ref<Obj> = msg_send!(
        metal_cpp_rs::objc_abi::Obj::get("MTLCaptureManager\0".as_ptr()).unwrap(),
        mgr_sel
    );

    // Create an NSURL from the path string.
    let path_str = NSString::from_str(std::ffi::CStr::from_ptr(path).to_str().unwrap());
    let url: Ref<Obj> = msg_send!(
        metal_cpp_rs::objc_abi::Obj::get("NSURL\0".as_ptr()).unwrap(),
        Sel::get("fileURLWithPath:\0").as_ptr().unwrap(),
        path_str
    );

    // Create an MTLCaptureDescriptor.
    let desc_cls = metal_cpp_rs::objc_abi::Obj::get("MTLCaptureDescriptor\0".as_ptr()).unwrap();
    let desc: Ref<Obj> = msg_send!(desc_cls, Sel::get("alloc\0").as_ptr().unwrap());
    let desc: Ref<Obj> = msg_send!(desc, Sel::get("init\0").as_ptr().unwrap());

    // Set capture object (the device).
    let _: () = msg_send!(
        desc,
        Sel::get("setCaptureObject:\0").as_ptr().unwrap(),
        device
    );
    // Set destination to GPU trace document (file).
    // MTLCaptureDestinationGPUTraceDocument = 1
    let _: () = msg_send!(desc, Sel::get("setDestination:\0").as_ptr().unwrap(), 1u64);
    // Set output URL.
    let _: () = msg_send!(desc, Sel::get("setOutputURL:\0").as_ptr().unwrap(), url);

    // Start capture.
    let mut error = Strong::<MTLError>::nil();
    let ok: bool = msg_send!(
        mgr,
        Sel::get("startCaptureWithDescriptor:error:\0")
            .as_ptr()
            .unwrap(),
        desc,
        &mut error
    );
    if !ok {
        eprintln!(
            "Capture start failed: {}",
            if error.is_nil() {
                "(unknown)".to_string()
            } else {
                error.localizedDescription().to_string()
            }
        );
        false
    } else {
        true
    }
}

/// Stop the active GPU capture.
unsafe fn stop_capture() {
    let mgr_sel = Sel::get("sharedCaptureManager\0").as_ptr().unwrap();
    let mgr: Ref<Obj> = msg_send!(
        metal_cpp_rs::objc_abi::Obj::get("MTLCaptureManager\0".as_ptr()).unwrap(),
        mgr_sel
    );
    let _: () = msg_send!(mgr, Sel::get("stopCapture\0").as_ptr().unwrap());
}

unsafe fn build_pipeline(
    device: Ref<MTLDevice>,
    src: &str,
    vs: &str,
    fs: &str,
) -> Strong<MTLRenderPipelineState> {
    let library = device.newLibrary(
        NSString::from_str(src).as_ref(),
        MTLCompileOptions::new().as_ref(),
        &mut Strong::<MTLError>::nil(),
    );
    let desc = MTLRenderPipelineDescriptor::new();
    desc.setVertexFunction(
        library
            .newFunction(NSString::from_str(vs).as_ref())
            .as_ref(),
    );
    desc.setFragmentFunction(
        library
            .newFunction(NSString::from_str(fs).as_ref())
            .as_ref(),
    );
    desc.colorAttachments()
        .objectAtIndexedSubscript(0)
        .setPixelFormat(MTLPixelFormat::PixelFormatBGRA8Unorm);
    device.newRenderPipelineState(desc.as_ref(), &mut Strong::<MTLError>::nil())
}

unsafe fn upload<T: Copy>(device: Ref<MTLDevice>, data: &[T]) -> Strong<MTLBuffer> {
    let buf = device.newBuffer(
        (std::mem::size_of::<T>() * data.len()) as u64,
        MTLResourceOptions::ResourceStorageModeShared,
    );
    std::ptr::copy_nonoverlapping(data.as_ptr(), buf.contents() as *mut T, data.len());
    buf
}
