//! `MTL::CommandQueue` + `MTL::CommandQueueDescriptor`
//! (`Metal/MTLCommandQueue.hpp`).

use crate::{
    foundation::NSString,
    metal::{
        command_buffer::{MTLCommandBuffer, MTLCommandBufferDescriptor},
        device::MTLDevice,
    },
    objc_abi::{NSUInteger, Obj, Raw, Ref, Strong, objc_class, sel, send, send1},
};

objc_class!(
    /// `MTL::CommandQueueDescriptor`.
    pub struct MTLCommandQueueDescriptor : "MTLCommandQueueDescriptor";
);

impl MTLCommandQueueDescriptor {
    /// `-maxCommandBufferCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxCommandBufferCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxCommandBufferCount"))
    }

    /// `-setMaxCommandBufferCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMaxCommandBufferCount(&self, maxCommandBufferCount: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setMaxCommandBufferCount:"),
            maxCommandBufferCount,
        );
    }
}

objc_class!(
    /// `MTL::CommandQueue` (`Metal/MTLCommandQueue.hpp`).
    pub struct MTLCommandQueue : "MTLCommandQueue";
);

impl MTLCommandQueue {
    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }

    /// `-setLabel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLabel(&self, label: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setLabel:"), label.as_raw());
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-commandBuffer` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commandBuffer(&self) -> Strong<MTLCommandBuffer> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("commandBuffer")))
    }

    /// `-commandBufferWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commandBufferWithDescriptor(
        &self,
        descriptor: Ref<MTLCommandBufferDescriptor>,
    ) -> Strong<MTLCommandBuffer> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("commandBufferWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-commandBufferWithUnretainedReferences`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commandBufferWithUnretainedReferences(&self) -> Strong<MTLCommandBuffer> {
        Strong::from_retain(send::<Raw>(
            self.as_raw(),
            sel!("commandBufferWithUnretainedReferences"),
        ))
    }

    /// `-insertDebugCaptureBoundary`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn insertDebugCaptureBoundary(&self) {
        send::<()>(self.as_raw(), sel!("insertDebugCaptureBoundary"));
    }
}
