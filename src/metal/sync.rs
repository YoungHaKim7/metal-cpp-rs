//! Synchronization — `MTL::Event`, `MTL::SharedEvent`, `MTL::Fence`
//! (`Metal/MTLEvent.hpp`, `Metal/MTLFence.hpp`).

use crate::foundation::NSString;
use crate::metal::device::MTLDevice;
use crate::objc_abi::{Obj, Raw, Ref, objc_class, sel, send, send1, send2};

objc_class!(
    /// `MTL::Event` (`Metal/MTLEvent.hpp`).
    pub struct MTLEvent : "MTLEvent";
);

impl MTLEvent {
    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
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
}

objc_class!(
    /// `MTL::SharedEvent` (`Metal/MTLEvent.hpp`).
    pub struct MTLSharedEvent : "MTLSharedEvent";
);

impl MTLSharedEvent {
    /// `-signaledValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn signaledValue(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("signaledValue"))
    }

    /// `-setSignaledValue:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSignaledValue(&self, signaledValue: u64) {
        send1::<u64, ()>(self.as_raw(), sel!("setSignaledValue:"), signaledValue);
    }

    /// `-waitUntilSignaledValue:timeoutMS:` (`BOOL` return; block-free wait).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitUntilSignaledValue(&self, value: u64, timeoutMS: u64) -> bool {
        send2_bool_u64(
            self.as_raw(),
            sel!("waitUntilSignaledValue:timeoutMS:"),
            value,
            timeoutMS,
        )
    }

    /// `-device` (inherited from `MTL::Event`)
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-label` / `-setLabel:` (inherited from `MTL::Event`)
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
}

/// `-waitUntilSignaledValue:timeoutMS:` returns `BOOL`.
unsafe fn send2_bool_u64(receiver: Raw, sel: crate::objc_abi::Sel, a1: u64, a2: u64) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        send2::<u64, u64, bool>(receiver, sel, a1, a2)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        send2::<u64, u64, i8>(receiver, sel, a1, a2) != 0
    }
}

objc_class!(
    /// `MTL::Fence` (`Metal/MTLFence.hpp`).
    pub struct MTLFence : "MTLFence";
);

impl MTLFence {
    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
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
}
