//! The Metal module — the Rust conversion of `Metal/Metal.hpp`.
//!
//! Free functions in this module are the Metal framework C entry points
//! (`MTLCreateSystemDefaultDevice`, `MTLCopyAllDevices`, ...) that metal-cpp
//! wraps as `MTL::CreateSystemDefaultDevice` etc.

pub mod argument;
pub mod capture;
pub mod command_buffer;
pub mod command_queue;
pub mod device;
pub mod encoders;
pub mod enums;
pub mod pipeline;
pub mod renderpass;
pub mod resources;
pub mod shader;
pub mod sync;
pub mod types;

pub use argument::*;
pub use capture::*;
pub use command_buffer::*;
pub use command_queue::*;
pub use device::*;
pub use encoders::*;
pub use enums::*;
pub use pipeline::*;
pub use renderpass::*;
pub use resources::*;
pub use shader::*;
pub use sync::*;
pub use types::*;

use crate::foundation::NSArray;
use crate::objc_abi::{Obj, Raw, Ref, Strong};

// Link the Metal framework (this also guarantees `MTLDevice` and friends are
// registered in the ObjC runtime at process start).
#[cfg_attr(target_vendor = "apple", link(name = "Metal", kind = "framework"))]
unsafe extern "C" {
    #[link_name = "MTLCreateSystemDefaultDevice"]
    fn MTLCreateSystemDefaultDevice_c() -> Raw;
    #[link_name = "MTLCopyAllDevices"]
    fn MTLCopyAllDevices_c() -> Raw;
    #[link_name = "MTLCopyAllDevicesWithObserver"]
    fn MTLCopyAllDevicesWithObserver_c(pOutObserver: *mut Raw, handler: Raw) -> Raw;
    #[link_name = "MTLRemoveDeviceObserver"]
    fn MTLRemoveDeviceObserver_c(observer: Raw);
}

/// `MTL::CreateSystemDefaultDevice()` — the default GPU for the process.
///
/// The returned object is retained into a [`Strong`] (the C function hands
/// back an autoreleased object; metal-cpp's `RetainPtr` convention applies).
///
/// # Safety
/// Calls the Metal C function — Apple platforms only.
pub unsafe fn MTLCreateSystemDefaultDevice() -> Strong<MTLDevice> {
    Strong::from_retain(MTLCreateSystemDefaultDevice_c())
}

/// `MTL::CopyAllDevices()` — every Metal device in the system.
/// Follows the Copy rule (+1), so ownership is transferred.
///
/// # Safety
/// Calls the Metal C function — Apple platforms only.
pub unsafe fn MTLCopyAllDevices() -> Strong<NSArray<MTLDevice>> {
    Strong::from_transfer(MTLCopyAllDevices_c())
}

/// `MTL::CopyAllDevicesWithObserver(outObserver, handler)` — `handler` is a
/// raw block (`void (^)(id <MTLDevice> device, MTLDeviceNotificationName name)`).
///
/// # Safety
/// Calls the Metal C function. The observer must later be released with
/// [`MTLRemoveDeviceObserver`].
pub unsafe fn MTLCopyAllDevicesWithObserver(
    pOutObserver: *mut Raw,
    handler: Raw,
) -> Strong<NSArray<MTLDevice>> {
    Strong::from_transfer(MTLCopyAllDevicesWithObserver_c(pOutObserver, handler))
}

/// `MTL::RemoveDeviceObserver(observer)`.
///
/// # Safety
/// Calls the Metal C function.
pub unsafe fn MTLRemoveDeviceObserver(observer: Ref<crate::foundation::NSObject>) {
    MTLRemoveDeviceObserver_c(observer.as_raw())
}
