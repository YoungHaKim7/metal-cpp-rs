//! `MTL::CommandBuffer` + `MTL::CommandBufferDescriptor`
//! (`Metal/MTLCommandBuffer.hpp`).

use crate::blocks::Block1;
use crate::foundation::{NSError, NSString};
use crate::metal::command_queue::MTLCommandQueue;
use crate::metal::device::MTLDevice;
use crate::metal::encoders::{
    MTLBlitCommandEncoder, MTLComputeCommandEncoder, MTLParallelRenderCommandEncoder,
    MTLRenderCommandEncoder,
};
use crate::metal::enums::{MTLCommandBufferErrorOption, MTLCommandBufferStatus, MTLDispatchType};
use crate::metal::renderpass::MTLRenderPassDescriptor;
use crate::metal::sync::MTLEvent;
use crate::objc_abi::{
    CFTimeInterval, Obj, Raw, Ref, Strong, bool_arg, send, send_bool, send1, send2,
};
use crate::objc_abi::{objc_class, sel};

objc_class!(
    /// `MTL::CommandBufferDescriptor`.
    pub struct MTLCommandBufferDescriptor : "MTLCommandBufferDescriptor";
);

impl MTLCommandBufferDescriptor {
    /// `-errorOptions`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption {
        send::<MTLCommandBufferErrorOption>(self.as_raw(), sel!("errorOptions"))
    }

    /// `-setErrorOptions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setErrorOptions(&self, errorOptions: MTLCommandBufferErrorOption) {
        send1::<MTLCommandBufferErrorOption, ()>(
            self.as_raw(),
            sel!("setErrorOptions:"),
            errorOptions,
        );
    }

    /// `-retainedReferences`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn retainedReferences(&self) -> bool {
        send_bool(self.as_raw(), sel!("retainedReferences"))
    }

    /// `-setRetainedReferences:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRetainedReferences(&self, retainedReferences: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setRetainedReferences:"),
            bool_arg(retainedReferences),
        );
    }
}

objc_class!(
    /// `MTL::CommandBuffer` (`Metal/MTLCommandBuffer.hpp`).
    pub struct MTLCommandBuffer : "MTLCommandBuffer";
);

impl MTLCommandBuffer {
    // -- properties ----------------------------------------------------------

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

    /// `-commandQueue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commandQueue(&self) -> Ref<MTLCommandQueue> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("commandQueue")))
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-status`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn status(&self) -> MTLCommandBufferStatus {
        send::<MTLCommandBufferStatus>(self.as_raw(), sel!("status"))
    }

    /// `-error`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn error(&self) -> Ref<NSError> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("error")))
    }

    /// `-retainedReferences`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn retainedReferences(&self) -> bool {
        send_bool(self.as_raw(), sel!("retainedReferences"))
    }

    /// `-errorOptions`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption {
        send::<MTLCommandBufferErrorOption>(self.as_raw(), sel!("errorOptions"))
    }

    /// `-GPUStartTime`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn GPUStartTime(&self) -> CFTimeInterval {
        send::<CFTimeInterval>(self.as_raw(), sel!("GPUStartTime"))
    }

    /// `-GPUEndTime`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn GPUEndTime(&self) -> CFTimeInterval {
        send::<CFTimeInterval>(self.as_raw(), sel!("GPUEndTime"))
    }

    /// `-kernelStartTime`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn kernelStartTime(&self) -> CFTimeInterval {
        send::<CFTimeInterval>(self.as_raw(), sel!("kernelStartTime"))
    }

    /// `-kernelEndTime`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn kernelEndTime(&self) -> CFTimeInterval {
        send::<CFTimeInterval>(self.as_raw(), sel!("kernelEndTime"))
    }

    // -- lifetime ------------------------------------------------------------

    /// `-enqueue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn enqueue(&self) {
        send::<()>(self.as_raw(), sel!("enqueue"));
    }

    /// `-commit`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn commit(&self) {
        send::<()>(self.as_raw(), sel!("commit"));
    }

    /// `-waitUntilScheduled`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitUntilScheduled(&self) {
        send::<()>(self.as_raw(), sel!("waitUntilScheduled"));
    }

    /// `-waitUntilCompleted`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitUntilCompleted(&self) {
        send::<()>(self.as_raw(), sel!("waitUntilCompleted"));
    }

    // -- handlers (blocks) ----------------------------------------------------

    /// `-addScheduledHandler:` — block receives `(MTLCommandBuffer*, void*)`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn addScheduledHandler<F: FnMut(Raw) + 'static>(&self, block: &Block1<F>) {
        send1::<Raw, ()>(self.as_raw(), sel!("addScheduledHandler:"), block.as_raw());
    }

    /// `-addCompletedHandler:` — block receives `(MTLCommandBuffer*, void*)`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn addCompletedHandler<F: FnMut(Raw) + 'static>(&self, block: &Block1<F>) {
        send1::<Raw, ()>(self.as_raw(), sel!("addCompletedHandler:"), block.as_raw());
    }

    // -- debug ---------------------------------------------------------------

    /// `-pushDebugGroup:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn pushDebugGroup(&self, string: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("pushDebugGroup:"), string.as_raw());
    }

    /// `-popDebugGroup`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn popDebugGroup(&self) {
        send::<()>(self.as_raw(), sel!("popDebugGroup"));
    }

    /// `-insertDebugSignpost:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn insertDebugSignpost(&self, string: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("insertDebugSignpost:"), string.as_raw());
    }

    // -- encoders ------------------------------------------------------------

    /// `-renderCommandEncoderWithDescriptor:` (autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderCommandEncoder(
        &self,
        renderPassDescriptor: Ref<MTLRenderPassDescriptor>,
    ) -> Strong<MTLRenderCommandEncoder> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("renderCommandEncoderWithDescriptor:"),
            renderPassDescriptor.as_raw(),
        ))
    }

    /// `-parallelRenderCommandEncoderWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn parallelRenderCommandEncoder(
        &self,
        renderPassDescriptor: Ref<MTLRenderPassDescriptor>,
    ) -> Strong<MTLParallelRenderCommandEncoder> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("parallelRenderCommandEncoderWithDescriptor:"),
            renderPassDescriptor.as_raw(),
        ))
    }

    /// `-computeCommandEncoder`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn computeCommandEncoder(&self) -> Strong<MTLComputeCommandEncoder> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("computeCommandEncoder")))
    }

    /// `-computeCommandEncoderWithDispatchType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn computeCommandEncoderWithDispatchType(
        &self,
        dispatchType: MTLDispatchType,
    ) -> Strong<MTLComputeCommandEncoder> {
        Strong::from_retain(send1::<MTLDispatchType, Raw>(
            self.as_raw(),
            sel!("computeCommandEncoderWithDispatchType:"),
            dispatchType,
        ))
    }

    /// `-blitCommandEncoder`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn blitCommandEncoder(&self) -> Strong<MTLBlitCommandEncoder> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("blitCommandEncoder")))
    }

    // -- presentation / synchronization --------------------------------------

    /// `-presentDrawable:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn presentDrawable(&self, drawable: Raw) {
        send1::<Raw, ()>(self.as_raw(), sel!("presentDrawable:"), drawable);
    }

    /// `-presentDrawable:atTime:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn presentDrawableAtTime(&self, drawable: Raw, presentationTime: CFTimeInterval) {
        send2::<Raw, CFTimeInterval, ()>(
            self.as_raw(),
            sel!("presentDrawable:atTime:"),
            drawable,
            presentationTime,
        );
    }

    /// `-presentDrawable:afterMinimumDuration:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn presentDrawableAfterMinimumDuration(
        &self,
        drawable: Raw,
        duration: CFTimeInterval,
    ) {
        send2::<Raw, CFTimeInterval, ()>(
            self.as_raw(),
            sel!("presentDrawable:afterMinimumDuration:"),
            drawable,
            duration,
        );
    }

    /// `-encodeWaitForEvent:value:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn encodeWait(&self, event: Ref<MTLEvent>, value: u64) {
        send2::<Raw, u64, ()>(
            self.as_raw(),
            sel!("encodeWaitForEvent:value:"),
            event.as_raw(),
            value,
        );
    }

    /// `-encodeSignalEvent:value:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn encodeSignalEvent(&self, event: Ref<MTLEvent>, value: u64) {
        send2::<Raw, u64, ()>(
            self.as_raw(),
            sel!("encodeSignalEvent:value:"),
            event.as_raw(),
            value,
        );
    }

    // -- residency (pass-through, residency sets are opaque in core scope) ----

    /// `-useResidencySet:` (raw residency-set pointer).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useResidencySet(&self, residencySet: Raw) {
        send1::<Raw, ()>(self.as_raw(), sel!("useResidencySet:"), residencySet);
    }
}
