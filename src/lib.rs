//! # metal-cpp-rs
//!
//! A Rust conversion of Apple's [metal-cpp](https://github.com/apple/metal-cpp)
//! (macOS 27 / iOS 27 SDK era, Metal 4) implemented **concretely against the
//! Objective-C ABI** — the same approach metal-cpp itself uses when it bridges
//! C++ to the Objective-C runtime.
//!
//! ## How it works
//!
//! metal-cpp wraps every Metal class in a C++ class whose inline methods call
//! `objc_msgSend` with the exact Objective-C selector strings registered in
//! `MTLHeaderBridge.hpp` / `NSPrivate.hpp`. This crate does the same from Rust:
//!
//! * [`objc_abi`] declares the raw runtime ABI (`objc_msgSend`,
//!   `objc_msgSend_stret`, `objc_msgSend_fpret`, `objc_getClass`,
//!   `sel_registerName`, retain/release/autorelease, autorelease pool
//!   push/pop) via plain `unsafe extern "C"` — zero dependencies.
//! * Every wrapper method uses the **exact selector string** that metal-cpp's
//!   `_MTL_PRIVATE_SEL` call sites resolve to (e.g. `MTL::Device::newBuffer`
//!   sends `"newBufferWithLength:options:"`), so the ABI is 1:1 with the
//!   upstream C++ implementation.
//! * Struct layouts ([`metal::types`]) are `#[repr(C)]` mirrors of the packed
//!   C++ structs (`_MTL_PACKED`).
//! * Enum discriminants ([`metal::enums`]) are the exact values from the
//!   metal-cpp headers (e.g. `MTLPixelFormat::PixelFormatBGRA8Unorm = 80`).
//! * x86_64-specific ABI details are handled: `objc_msgSend_stret` for
//!   >16-byte struct returns, `objc_msgSend_fpret` for `float` returns, and
//!   the `BOOL` type difference between arm64 (`bool`) and x86_64
//!   (`signed char`).
//! * [`blocks`] implements ObjC **block literals** by hand
//!   (`_NSConcreteStackBlock`, `_Block_copy`, copy/dispose helpers), which
//!   enables completion-handler APIs such as `addCompletedHandler:` or
//!   `newLibraryWithSource:options:completionHandler:`.
//! * Ownership follows metal-cpp's `NS::RetainPtr` / `NS::TransferPtr`
//!   conventions: methods that create objects return
//!   [`Strong`](objc_abi::Strong) (owning, releases on drop), property getters
//!   return [`Ref`](objc_abi::Ref) (non-owning view, like the raw C++ pointer
//!   metal-cpp returns).
//!
//! ## Platform
//!
//! Apple platforms only (macOS 10.11+ / iOS 8+ APIs; Metal 4 APIs require
//! macOS 26+). The crate links `libobjc` plus the `Foundation`, `Metal` and
//! `QuartzCore` frameworks. `cargo check` works on any host; building and
//! running the examples requires a Mac.
//!
//! ## Version
//!
//! Crate version `0.1.0` — matching the requested Rust toolchain version
//! (rustc 1.100.0 stable, edition 2024, `rust-version = "1.100.0"`).

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::too_many_arguments,
    clippy::missing_safety_doc,
    clippy::new_without_default
)]

pub mod blocks;
pub mod foundation;
pub mod metal;
pub mod objc_abi;
pub mod quartz_core;

/// Everything you typically need in scope, mirroring the flat namespace of
/// `Metal/Metal.hpp` + `Foundation/Foundation.hpp` + `QuartzCore/QuartzCore.hpp`.
pub mod prelude {
    pub use crate::blocks::{Block0, Block1, Block2};
    pub use crate::foundation::*;
    pub use crate::metal::*;
    pub use crate::objc_abi::{Class, Obj, Ref, Sel, Strong, autoreleasepool};
    pub use crate::quartz_core::*;
}
