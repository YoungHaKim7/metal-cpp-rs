//! Render-pass descriptors — `MTL::RenderPassDescriptor` and the attachment
//! descriptor family (`Metal/MTLRenderPass.hpp`).

use crate::metal::enums::{
    MTLLoadAction, MTLMultisampleDepthResolveFilter, MTLMultisampleStencilResolveFilter,
    MTLStoreAction, MTLStoreActionOptions, MTLVisibilityResultType,
};
use crate::metal::resources::MTLBuffer;
use crate::metal::types::{MTLClearColor, MTLSamplePosition};
use crate::objc_abi::{
    NSUInteger, Obj, Raw, Ref, Strong, bool_arg, send, send_bool, send_stret, send1, send2,
};
use crate::objc_abi::{objc_class, sel};

objc_class!(
    /// `MTL::RenderPassAttachmentDescriptor` — base of color/depth/stencil
    /// attachment descriptors.
    pub struct MTLRenderPassAttachmentDescriptor : "MTLRenderPassAttachmentDescriptor";
);

impl MTLRenderPassAttachmentDescriptor {
    /// `-texture` / `-setTexture:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn texture(&self) -> Ref<crate::metal::resources::MTLTexture> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("texture")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTexture(&self, texture: Ref<crate::metal::resources::MTLTexture>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setTexture:"), texture.as_raw());
    }

    /// `-resolveTexture` / `-setResolveTexture:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resolveTexture(&self) -> Ref<crate::metal::resources::MTLTexture> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("resolveTexture")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResolveTexture(
        &self,
        resolveTexture: Ref<crate::metal::resources::MTLTexture>,
    ) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setResolveTexture:"),
            resolveTexture.as_raw(),
        );
    }

    /// `-level` / `-setLevel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn level(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("level"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLevel(&self, level: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setLevel:"), level);
    }

    /// `-slice` / `-setSlice:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn slice(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("slice"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSlice(&self, slice: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setSlice:"), slice);
    }

    /// `-depthPlane` / `-setDepthPlane:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthPlane(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("depthPlane"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthPlane(&self, depthPlane: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setDepthPlane:"), depthPlane);
    }

    /// `-resolveLevel` / `-setResolveLevel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resolveLevel(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("resolveLevel"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResolveLevel(&self, resolveLevel: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setResolveLevel:"), resolveLevel);
    }

    /// `-resolveSlice` / `-setResolveSlice:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resolveSlice(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("resolveSlice"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResolveSlice(&self, resolveSlice: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setResolveSlice:"), resolveSlice);
    }

    /// `-resolveDepthPlane` / `-setResolveDepthPlane:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resolveDepthPlane(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("resolveDepthPlane"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResolveDepthPlane(&self, resolveDepthPlane: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setResolveDepthPlane:"),
            resolveDepthPlane,
        );
    }

    /// `-loadAction` / `-setLoadAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn loadAction(&self) -> MTLLoadAction {
        send::<MTLLoadAction>(self.as_raw(), sel!("loadAction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLoadAction(&self, loadAction: MTLLoadAction) {
        send1::<MTLLoadAction, ()>(self.as_raw(), sel!("setLoadAction:"), loadAction);
    }

    /// `-storeAction` / `-setStoreAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn storeAction(&self) -> MTLStoreAction {
        send::<MTLStoreAction>(self.as_raw(), sel!("storeAction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStoreAction(&self, storeAction: MTLStoreAction) {
        send1::<MTLStoreAction, ()>(self.as_raw(), sel!("setStoreAction:"), storeAction);
    }

    /// `-storeActionOptions` / `-setStoreActionOptions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn storeActionOptions(&self) -> MTLStoreActionOptions {
        send::<MTLStoreActionOptions>(self.as_raw(), sel!("storeActionOptions"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStoreActionOptions(&self, storeActionOptions: MTLStoreActionOptions) {
        send1::<MTLStoreActionOptions, ()>(
            self.as_raw(),
            sel!("setStoreActionOptions:"),
            storeActionOptions,
        );
    }
}

objc_class!(
    /// `MTL::RenderPassColorAttachmentDescriptor`.
    pub struct MTLRenderPassColorAttachmentDescriptor : "MTLRenderPassColorAttachmentDescriptor";
);

impl MTLRenderPassColorAttachmentDescriptor {
    /// `-clearColor` — 32-byte struct return ⇒ `objc_msgSend_stret` on x86_64.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn clearColor(&self) -> MTLClearColor {
        send_stret::<MTLClearColor>(self.as_raw(), sel!("clearColor"))
    }

    /// `-setClearColor:` (struct passed by value — no special variant).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setClearColor(&self, clearColor: MTLClearColor) {
        send1::<MTLClearColor, ()>(self.as_raw(), sel!("setClearColor:"), clearColor);
    }

    /// Non-owning upcast to the attachment base.
    pub unsafe fn as_attachment(&self) -> Ref<MTLRenderPassAttachmentDescriptor> {
        Ref::from_raw(self.as_raw())
    }
}

objc_class!(
    /// `MTL::RenderPassDepthAttachmentDescriptor`.
    pub struct MTLRenderPassDepthAttachmentDescriptor : "MTLRenderPassDepthAttachmentDescriptor";
);

impl MTLRenderPassDepthAttachmentDescriptor {
    /// `-clearDepth` / `-setClearDepth:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn clearDepth(&self) -> f64 {
        send::<f64>(self.as_raw(), sel!("clearDepth"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setClearDepth(&self, clearDepth: f64) {
        send1::<f64, ()>(self.as_raw(), sel!("setClearDepth:"), clearDepth);
    }

    /// `-depthResolveFilter` / `-setDepthResolveFilter:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthResolveFilter(&self) -> MTLMultisampleDepthResolveFilter {
        send::<MTLMultisampleDepthResolveFilter>(self.as_raw(), sel!("depthResolveFilter"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthResolveFilter(
        &self,
        depthResolveFilter: MTLMultisampleDepthResolveFilter,
    ) {
        send1::<MTLMultisampleDepthResolveFilter, ()>(
            self.as_raw(),
            sel!("setDepthResolveFilter:"),
            depthResolveFilter,
        );
    }

    /// Non-owning upcast to the attachment base.
    pub unsafe fn as_attachment(&self) -> Ref<MTLRenderPassAttachmentDescriptor> {
        Ref::from_raw(self.as_raw())
    }
}

objc_class!(
    /// `MTL::RenderPassStencilAttachmentDescriptor`.
    pub struct MTLRenderPassStencilAttachmentDescriptor : "MTLRenderPassStencilAttachmentDescriptor";
);

impl MTLRenderPassStencilAttachmentDescriptor {
    /// `-clearStencil` / `-setClearStencil:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn clearStencil(&self) -> u32 {
        send::<u32>(self.as_raw(), sel!("clearStencil"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setClearStencil(&self, clearStencil: u32) {
        send1::<u32, ()>(self.as_raw(), sel!("setClearStencil:"), clearStencil);
    }

    /// `-stencilResolveFilter` / `-setStencilResolveFilter:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stencilResolveFilter(&self) -> MTLMultisampleStencilResolveFilter {
        send::<MTLMultisampleStencilResolveFilter>(self.as_raw(), sel!("stencilResolveFilter"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilResolveFilter(
        &self,
        stencilResolveFilter: MTLMultisampleStencilResolveFilter,
    ) {
        send1::<MTLMultisampleStencilResolveFilter, ()>(
            self.as_raw(),
            sel!("setStencilResolveFilter:"),
            stencilResolveFilter,
        );
    }

    /// Non-owning upcast to the attachment base.
    pub unsafe fn as_attachment(&self) -> Ref<MTLRenderPassAttachmentDescriptor> {
        Ref::from_raw(self.as_raw())
    }
}

objc_class!(
    /// `MTL::RenderPassColorAttachmentDescriptorArray`.
    pub struct MTLRenderPassColorAttachmentDescriptorArray : "MTLRenderPassColorAttachmentDescriptorArray";
);

impl MTLRenderPassColorAttachmentDescriptorArray {
    /// `-objectAtIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectAtIndexedSubscript(
        &self,
        attachmentIndex: NSUInteger,
    ) -> Ref<MTLRenderPassColorAttachmentDescriptor> {
        Ref::from_raw(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("objectAtIndexedSubscript:"),
            attachmentIndex,
        ))
    }

    /// `-setObject:atIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setObjectAtIndexedSubscript(
        &self,
        attachment: Ref<MTLRenderPassColorAttachmentDescriptor>,
        attachmentIndex: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setObject:atIndexedSubscript:"),
            attachment.as_raw(),
            attachmentIndex,
        );
    }
}

objc_class!(
    /// `MTL::RenderPassDescriptor` (`Metal/MTLRenderPass.hpp`).
    pub struct MTLRenderPassDescriptor : "MTLRenderPassDescriptor";
);

impl MTLRenderPassDescriptor {
    /// `+[MTLRenderPassDescriptor renderPassDescriptor]` (class factory,
    /// autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderPassDescriptor() -> Strong<MTLRenderPassDescriptor> {
        Strong::from_retain(send::<Raw>(
            MTLRenderPassDescriptor::class().ptr(),
            sel!("renderPassDescriptor"),
        ))
    }

    /// `-colorAttachments`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn colorAttachments(&self) -> Ref<MTLRenderPassColorAttachmentDescriptorArray> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("colorAttachments")))
    }

    /// `-depthAttachment`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthAttachment(&self) -> Ref<MTLRenderPassDepthAttachmentDescriptor> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("depthAttachment")))
    }

    /// `-stencilAttachment`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stencilAttachment(&self) -> Ref<MTLRenderPassStencilAttachmentDescriptor> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("stencilAttachment")))
    }

    /// `-renderTargetWidth` / `-setRenderTargetWidth:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderTargetWidth(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("renderTargetWidth"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRenderTargetWidth(&self, renderTargetWidth: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setRenderTargetWidth:"),
            renderTargetWidth,
        );
    }

    /// `-renderTargetHeight` / `-setRenderTargetHeight:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderTargetHeight(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("renderTargetHeight"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRenderTargetHeight(&self, renderTargetHeight: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setRenderTargetHeight:"),
            renderTargetHeight,
        );
    }

    /// `-renderTargetArrayLength` / `-setRenderTargetArrayLength:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderTargetArrayLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("renderTargetArrayLength"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRenderTargetArrayLength(&self, renderTargetArrayLength: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setRenderTargetArrayLength:"),
            renderTargetArrayLength,
        );
    }

    /// `-defaultRasterSampleCount` / `-setDefaultRasterSampleCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn defaultRasterSampleCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("defaultRasterSampleCount"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDefaultRasterSampleCount(&self, defaultRasterSampleCount: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setDefaultRasterSampleCount:"),
            defaultRasterSampleCount,
        );
    }

    /// `-visibilityResultBuffer` / `-setVisibilityResultBuffer:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn visibilityResultBuffer(&self) -> Ref<MTLBuffer> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("visibilityResultBuffer")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVisibilityResultBuffer(&self, visibilityResultBuffer: Ref<MTLBuffer>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setVisibilityResultBuffer:"),
            visibilityResultBuffer.as_raw(),
        );
    }

    /// `-visibilityResultType` / `-setVisibilityResultType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn visibilityResultType(&self) -> MTLVisibilityResultType {
        send::<MTLVisibilityResultType>(self.as_raw(), sel!("visibilityResultType"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVisibilityResultType(&self, visibilityResultType: MTLVisibilityResultType) {
        send1::<MTLVisibilityResultType, ()>(
            self.as_raw(),
            sel!("setVisibilityResultType:"),
            visibilityResultType,
        );
    }

    /// `-getSamplePositions:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn getSamplePositions(
        &self,
        positions: *mut MTLSamplePosition,
        count: NSUInteger,
    ) -> NSUInteger {
        send2::<*mut MTLSamplePosition, NSUInteger, NSUInteger>(
            self.as_raw(),
            sel!("getSamplePositions:count:"),
            positions,
            count,
        )
    }

    /// `-setSamplePositions:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSamplePositions(
        &self,
        positions: *const MTLSamplePosition,
        count: NSUInteger,
    ) {
        send2::<*const MTLSamplePosition, NSUInteger, ()>(
            self.as_raw(),
            sel!("setSamplePositions:count:"),
            positions,
            count,
        );
    }

    /// `-supportColorAttachmentMapping` / `-setSupportColorAttachmentMapping:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportColorAttachmentMapping(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportColorAttachmentMapping"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSupportColorAttachmentMapping(&self, supportColorAttachmentMapping: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setSupportColorAttachmentMapping:"),
            bool_arg(supportColorAttachmentMapping),
        );
    }
}
