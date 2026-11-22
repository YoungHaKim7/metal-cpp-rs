//! Integration tests — Apple platforms only (they need the Objective-C
//! runtime and a Metal device). Run on a Mac with:
//!
//! ```sh
//! cargo test --target aarch64-apple-darwin
//! ```
#![cfg(target_vendor = "apple")]
#![allow(non_snake_case)]

use metal_cpp_rs::objc_abi::{NSUInteger, autoreleasepool};
use metal_cpp_rs::prelude::*;

#[test]
fn nsstring_roundtrip() {
    autoreleasepool(|| unsafe {
        let s = NSString::from_str("hello metal-cpp-rs");
        assert!(!s.is_nil());
        assert_eq!(s.to_string(), "hello metal-cpp-rs");
        let other = NSString::from_str("hello metal-cpp-rs");
        assert!(s.isEqualToString(&other));
    });
}

#[test]
fn nsarray_count_and_index() {
    autoreleasepool(|| unsafe {
        let a = NSString::from_str("a");
        let b = NSString::from_str("b");
        let arr = NSArray::arrayWithObjects(&[&*a, &*b]);
        assert_eq!(arr.count(), 2);
        assert_eq!(arr.objectAtIndex(1).to_string(), "b");
    });
}

#[test]
fn device_is_present_and_queue_works() {
    autoreleasepool(|| unsafe {
        let device = MTLCreateSystemDefaultDevice();
        assert!(!device.is_nil(), "no Metal device on this machine");
        let queue = device.newCommandQueue();
        assert!(!queue.is_nil());
        let cb = queue.commandBuffer();
        assert!(!cb.is_nil());
        assert_eq!(
            cb.status(),
            MTLCommandBufferStatus::CommandBufferStatusNotEnqueued
        );
        assert!(!device.name().to_string().is_empty());
        assert!(device.maxBufferLength() > 0);
    });
}

#[test]
fn buffer_contents_roundtrip() {
    autoreleasepool(|| unsafe {
        let device = MTLCreateSystemDefaultDevice();
        let data: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
        let buffer = device.newBufferWithBytes(
            data.as_ptr() as *const std::ffi::c_void,
            (data.len() * 4) as NSUInteger,
            MTLResourceOptions::ResourceStorageModeShared,
        );
        assert!(!buffer.is_nil());
        assert_eq!(buffer.length(), 16);
        let out = buffer.contents() as *const f32;
        assert_eq!(*out.add(0), 1.0);
        assert_eq!(*out.add(3), 4.0);
    });
}

#[test]
fn command_buffer_completed_handler_via_block_abi() {
    autoreleasepool(|| unsafe {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};
        let device = MTLCreateSystemDefaultDevice();
        let queue = device.newCommandQueue();
        let cb = queue.commandBuffer();
        let fired = Arc::new(AtomicU32::new(0));
        let fired2 = fired.clone();
        cb.addCompletedHandler(&Block1::new(move |_buffer| {
            fired2.fetch_add(1, Ordering::SeqCst);
        }));
        cb.commit();
        cb.waitUntilCompleted();
        assert_eq!(fired.load(Ordering::SeqCst), 1);
    });
}

#[test]
fn render_pass_attachment_struct_returns() {
    autoreleasepool(|| unsafe {
        let device = MTLCreateSystemDefaultDevice();
        let rp = MTLRenderPassDescriptor::renderPassDescriptor();
        let color0 = rp.colorAttachments().objectAtIndexedSubscript(0);
        color0.setClearColor(MTLClearColor::Make(0.25, 0.5, 0.75, 1.0));
        let cc = color0.clearColor();
        // Verifies the >16-byte struct return path (objc_msgSend_stret on x86_64).
        assert_eq!(cc.red, 0.25);
        assert_eq!(cc.green, 0.5);
        assert_eq!(cc.blue, 0.75);
        let _ = device; // silence unused in case no device is required here
    });
}
