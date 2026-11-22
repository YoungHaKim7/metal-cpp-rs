//! GPU capture — `MTL::CaptureManager`, `MTL::CaptureDescriptor`,
//! `MTL::CaptureScope` (`Metal/MTLCaptureManager.hpp`,
//! `Metal/MTLCaptureScope.hpp`).

use crate::foundation::{NSError, NSString, NSURL};
use crate::metal::command_queue::MTLCommandQueue;
use crate::metal::device::MTLDevice;
use crate::metal::enums::MTLCaptureDestination;
use crate::objc_abi::{objc_class, sel, send, send1, send1_bool, send2, send_bool, Obj, Raw, Ref, Strong};

objc_class!(
    /// `MTL::CaptureDescriptor` (`Metal/MTLCaptureManager.hpp`).
    pub struct MTLCaptureDescriptor : "MTLCaptureDescriptor";
);

impl MTLCaptureDescriptor {
    /// `-destination` / `-setDestination:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn destination(&self) -> MTLCaptureDestination {
        send::<MTLCaptureDestination>(self.as_raw(), sel!("destination"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDestination(&self, destination: MTLCaptureDestination) {
        send1::<MTLCaptureDestination, ()>(self.as_raw(), sel!("setDestination:"), destination);
    }

    /// `-outputURL` / `-setOutputURL:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn outputURL(&self) -> Ref<NSURL> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("outputURL")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setOutputURL(&self, outputURL: Ref<NSURL>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setOutputURL:"), outputURL.as_raw());
    }

    /// `-captureObject` / `-setCaptureObject:` (opaque: device / queue / scope).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn captureObject(&self) -> Raw {
        send::<Raw>(self.as_raw(), sel!("captureObject"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCaptureObject(&self, captureObject: Raw) {
        send1::<Raw, ()>(self.as_raw(), sel!("setCaptureObject:"), captureObject);
    }
}

objc_class!(
    /// `MTL::CaptureScope` (`Metal/MTLCaptureScope.hpp`).
    pub struct MTLCaptureScope : "MTLCaptureScope";
);

impl MTLCaptureScope {
    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-commandQueue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commandQueue(&self) -> Ref<MTLCommandQueue> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("commandQueue")))
    }

    /// `-label` / `-setLabel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLabel(&self, label: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setLabel:"), label.as_raw());
    }

    /// `-beginScope` (defined in `Metal/MTLPrivate.hpp`, not the bridge).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn beginScope(&self) {
        send::<()>(self.as_raw(), sel!("beginScope"));
    }

    /// `-endScope`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn endScope(&self) {
        send::<()>(self.as_raw(), sel!("endScope"));
    }
}

objc_class!(
    /// `MTL::CaptureManager` (`Metal/MTLCaptureManager.hpp`).
    pub struct MTLCaptureManager : "MTLCaptureManager";
);

impl MTLCaptureManager {
    /// `+[MTLCaptureManager sharedCaptureManager]` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sharedCaptureManager() -> Strong<MTLCaptureManager> {
        Strong::from_retain(send::<Raw>(
            MTLCaptureManager::class().ptr(),
            sel!("sharedCaptureManager"),
        ))
    }

    /// `-isCapturing`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isCapturing(&self) -> bool {
        send_bool(self.as_raw(), sel!("isCapturing"))
    }

    /// `-supportsDestination:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsDestination(&self, destination: MTLCaptureDestination) -> bool {
        send1_bool(self.as_raw(), sel!("supportsDestination:"), destination)
    }

    /// `-newCaptureScopeWithDevice:` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newCaptureScope(&self, device: Ref<MTLDevice>) -> Strong<MTLCaptureScope> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newCaptureScopeWithDevice:"),
            device.as_raw(),
        ))
    }

    /// `-newCaptureScopeWithCommandQueue:` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newCaptureScopeWithCommandQueue(
        &self,
        commandQueue: Ref<MTLCommandQueue>,
    ) -> Strong<MTLCaptureScope> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newCaptureScopeWithCommandQueue:"),
            commandQueue.as_raw(),
        ))
    }

    /// `-startCaptureWithDevice:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn startCaptureWithDevice(&self, device: Ref<MTLDevice>) {
        send1::<Raw, ()>(self.as_raw(), sel!("startCaptureWithDevice:"), device.as_raw());
    }

    /// `-startCaptureWithCommandQueue:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn startCaptureWithCommandQueue(&self, commandQueue: Ref<MTLCommandQueue>) {
        send1::<Raw, ()>(self.as_raw(), sel!("startCaptureWithCommandQueue:"), commandQueue.as_raw());
    }

    /// `-startCaptureWithScope:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn startCaptureWithScope(&self, captureScope: Ref<MTLCaptureScope>) {
        send1::<Raw, ()>(self.as_raw(), sel!("startCaptureWithScope:"), captureScope.as_raw());
    }

    /// `-startCaptureWithDescriptor:error:` — returns success.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn startCapture(
        &self,
        descriptor: Ref<MTLCaptureDescriptor>,
        error: &mut Strong<NSError>,
    ) -> bool {
        let mut err: Raw = std::ptr::null_mut();
        let ok = send2::<Raw, *mut Raw, crate::objc_abi::ObjBOOL>(
            self.as_raw(),
            sel!("startCaptureWithDescriptor:error:"),
            descriptor.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        #[cfg(target_arch = "aarch64")]
        {
            ok
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            ok != 0
        }
    }

    /// `-stopCapture`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stopCapture(&self) {
        send::<()>(self.as_raw(), sel!("stopCapture"));
    }

    /// `-defaultCaptureScope` / `-setDefaultCaptureScope:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn defaultCaptureScope(&self) -> Ref<MTLCaptureScope> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("defaultCaptureScope")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDefaultCaptureScope(&self, defaultCaptureScope: Ref<MTLCaptureScope>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setDefaultCaptureScope:"),
            defaultCaptureScope.as_raw(),
        );
    }
}
