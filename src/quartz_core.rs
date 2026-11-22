//! QuartzCore module — the Rust conversion of metal-cpp's
//! `QuartzCore/QuartzCore.hpp` (`CAMetalLayer.hpp`, `CAMetalDrawable.hpp`).
//!
//! Selectors match `QuartzCore/CAPrivate.hpp` verbatim
//! (`layer`, `setDevice:`, `setDrawableSize:`, `nextDrawable`, ...).
//! Note: `CAMetalDrawable` is an Objective-C *protocol* — you never create
//! one; you receive instances from [`CAMetalLayer::nextDrawable`].

use crate::{
    metal::{device::MTLDevice, enums::MTLPixelFormat, resources::MTLTexture},
    objc_abi::{
        CGFloat, NSUInteger, Obj, Raw, Ref, Strong, bool_arg, send, send_bool, send1,
        {objc_class, sel},
    },
};

/// `CGSize` from the system `CoreGraphics/CGGeometry.h` — `CGFloat` is
/// `double` on 64-bit Apple platforms (16 bytes; returned in registers, so
/// no `objc_msgSend_stret` is required for `-drawableSize`).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn Make(width: CGFloat, height: CGFloat) -> Self {
        CGSize { width, height }
    }
}

// Force-link QuartzCore by referencing the class symbol of CAMetalLayer (the
// framework exposes no stable C entry point we use; the class symbol keeps
// the dylib in the load list).
#[cfg_attr(target_vendor = "apple", link(name = "QuartzCore", kind = "framework"))]
unsafe extern "C" {
    #[link_name = "OBJC_CLASS_$_CAMetalLayer"]
    static _CAMETAL_LAYER_CLASS: u8;
}

/// Keeps the `OBJC_CLASS_$_CAMetalLayer` reference alive so the linker emits
/// `LC_LOAD_DYLIB` for QuartzCore. Returns the (opaque) address.
#[doc(hidden)]
pub fn quartz_core_force_link() -> usize {
    std::ptr::addr_of!(_CAMETAL_LAYER_CLASS) as usize
}

objc_class!(
    /// `CA::MetalLayer` (`QuartzCore/CAMetalLayer.hpp`) — the swapchain layer
    /// used to present Metal content.
    pub struct CAMetalLayer : "CAMetalLayer";
);

impl CAMetalLayer {
    /// `+[CAMetalLayer layer]` — class factory (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn layer() -> Strong<CAMetalLayer> {
        Strong::from_retain(send::<Raw>(CAMetalLayer::class().ptr(), sel!("layer")))
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-setDevice:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDevice(&self, device: Ref<MTLDevice>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setDevice:"), device.as_raw());
    }

    /// `-pixelFormat`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn pixelFormat(&self) -> MTLPixelFormat {
        send::<MTLPixelFormat>(self.as_raw(), sel!("pixelFormat"))
    }

    /// `-setPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPixelFormat(&self, pixelFormat: MTLPixelFormat) {
        send1::<MTLPixelFormat, ()>(self.as_raw(), sel!("setPixelFormat:"), pixelFormat);
    }

    /// `-framebufferOnly`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn framebufferOnly(&self) -> bool {
        send_bool(self.as_raw(), sel!("framebufferOnly"))
    }

    /// `-setFramebufferOnly:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFramebufferOnly(&self, framebufferOnly: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setFramebufferOnly:"),
            bool_arg(framebufferOnly),
        );
    }

    /// `-drawableSize` (16-byte `CGSize` — crosses the ABI in registers).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawableSize(&self) -> CGSize {
        send::<CGSize>(self.as_raw(), sel!("drawableSize"))
    }

    /// `-setDrawableSize:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDrawableSize(&self, drawableSize: CGSize) {
        send1::<CGSize, ()>(self.as_raw(), sel!("setDrawableSize:"), drawableSize);
    }

    /// `-nextDrawable` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn nextDrawable(&self) -> Strong<CAMetalDrawable> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("nextDrawable")))
    }

    /// `-maximumDrawableCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maximumDrawableCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maximumDrawableCount"))
    }

    /// `-setMaximumDrawableCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMaximumDrawableCount(&self, maximumDrawableCount: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setMaximumDrawableCount:"),
            maximumDrawableCount,
        );
    }

    /// `-displaySyncEnabled`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn displaySyncEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("displaySyncEnabled"))
    }

    /// `-setDisplaySyncEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDisplaySyncEnabled(&self, displaySyncEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setDisplaySyncEnabled:"),
            bool_arg(displaySyncEnabled),
        );
    }

    /// `-allowsNextDrawableTimeout`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn allowsNextDrawableTimeout(&self) -> bool {
        send_bool(self.as_raw(), sel!("allowsNextDrawableTimeout"))
    }

    /// `-setAllowsNextDrawableTimeout:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAllowsNextDrawableTimeout(&self, allowsNextDrawableTimeout: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setAllowsNextDrawableTimeout:"),
            bool_arg(allowsNextDrawableTimeout),
        );
    }

    /// `-wantsExtendedDynamicRangeContent`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn wantsExtendedDynamicRangeContent(&self) -> bool {
        send_bool(self.as_raw(), sel!("wantsExtendedDynamicRangeContent"))
    }

    /// `-setWantsExtendedDynamicRangeContent:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setWantsExtendedDynamicRangeContent(&self, wants: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setWantsExtendedDynamicRangeContent:"),
            bool_arg(wants),
        );
    }

    /// `-colorspace` (`CGColorSpaceRef` — opaque, from the CoreGraphics C API).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn colorspace(&self) -> Raw {
        send::<Raw>(self.as_raw(), sel!("colorspace"))
    }

    /// `-setColorspace:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setColorspace(&self, colorspace: Raw) {
        send1::<Raw, ()>(self.as_raw(), sel!("setColorspace:"), colorspace);
    }
}

objc_class!(
    /// `CA::MetalDrawable` (`QuartzCore/CAMetalDrawable.hpp`).
    ///
    /// ⚠️ Protocol-backed type: never call `alloc()`/`new()`/`class()` —
    /// obtain instances from [`CAMetalLayer::nextDrawable`].
    pub struct CAMetalDrawable : "CAMetalDrawable";
);

impl CAMetalDrawable {
    /// `-layer`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn layer(&self) -> Ref<CAMetalLayer> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("layer")))
    }

    /// `-texture`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn texture(&self) -> Ref<MTLTexture> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("texture")))
    }
}
