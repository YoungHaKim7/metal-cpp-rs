//! Command encoders — `MTL::CommandEncoder`, `MTL::RenderCommandEncoder`,
//! `MTL::ComputeCommandEncoder`, `MTL::BlitCommandEncoder` and
//! `MTL::ParallelRenderCommandEncoder` (`Metal/MTL*CommandEncoder.hpp`).
//!
//! All selectors are verbatim from metal-cpp's bridge, e.g.
//! `setVertexBuffer:offset:atIndex:`, `dispatchThreadgroups:threadsPerThreadgroup:`.

use std::ffi::c_void;

use crate::{
    foundation::{NSRange, NSString},
    metal::{
        device::MTLDevice,
        enums::{
            MTLBarrierScope, MTLCullMode, MTLDepthClipMode, MTLDispatchType, MTLIndexType,
            MTLPrimitiveType, MTLRenderStages, MTLResourceUsage, MTLStages, MTLStoreAction,
            MTLStoreActionOptions, MTLTriangleFillMode, MTLVisibilityResultMode, MTLWinding,
        },
        pipeline::{MTLComputePipelineState, MTLDepthStencilState, MTLRenderPipelineState},
        resources::{MTLBuffer, MTLHeap, MTLResource, MTLSamplerState, MTLTexture},
        sync::MTLFence,
        types::{MTLScissorRect, MTLSize, MTLViewport},
    },
    objc_abi::{
        NSUInteger, Obj, Raw, Ref, Strong, objc_class, sel, send, send1, send2, send3, send4,
        send5, send6, send7, send8, send9,
    },
};

objc_class!(
    /// `MTL::CommandEncoder` — base class of all encoders
    /// (`Metal/MTLCommandEncoder.hpp`).
    pub struct MTLCommandEncoder : "MTLCommandEncoder";
);

impl MTLCommandEncoder {
    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

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

    /// `-endEncoding`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn endEncoding(&self) {
        send::<()>(self.as_raw(), sel!("endEncoding"));
    }

    /// `-insertDebugSignpost:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn insertDebugSignpost(&self, string: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("insertDebugSignpost:"), string.as_raw());
    }

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

    /// `-barrierAfterQueueStages:beforeStages:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn barrierAfterQueueStages(
        &self,
        afterQueueStages: MTLStages,
        beforeStages: MTLStages,
    ) {
        send2::<MTLStages, MTLStages, ()>(
            self.as_raw(),
            sel!("barrierAfterQueueStages:beforeStages:"),
            afterQueueStages,
            beforeStages,
        );
    }
}

// ---------------------------------------------------------------------------
// Inherited MTL::CommandEncoder surface (C++ inheritance in metal-cpp).
// ---------------------------------------------------------------------------

/// Stamps the `MTL::CommandEncoder` inherited methods onto an encoder
/// subclass — the Rust equivalent of C++ inheritance from `MTL::CommandEncoder`.
macro_rules! encoder_base_methods {
    ($name:ident) => {
        impl $name {
            /// `-device` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn device(&self) -> Ref<MTLDevice> {
                Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
            }
            /// `-label` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn label(&self) -> Ref<NSString> {
                Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
            }
            /// `-setLabel:` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn setLabel(&self, label: Ref<NSString>) {
                send1::<Raw, ()>(self.as_raw(), sel!("setLabel:"), label.as_raw());
            }
            /// `-endEncoding` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn endEncoding(&self) {
                send::<()>(self.as_raw(), sel!("endEncoding"));
            }
            /// `-insertDebugSignpost:` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn insertDebugSignpost(&self, string: Ref<NSString>) {
                send1::<Raw, ()>(self.as_raw(), sel!("insertDebugSignpost:"), string.as_raw());
            }
            /// `-pushDebugGroup:` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn pushDebugGroup(&self, string: Ref<NSString>) {
                send1::<Raw, ()>(self.as_raw(), sel!("pushDebugGroup:"), string.as_raw());
            }
            /// `-popDebugGroup` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn popDebugGroup(&self) {
                send::<()>(self.as_raw(), sel!("popDebugGroup"));
            }
            /// `-barrierAfterQueueStages:beforeStages:` (inherited)
            ///
            /// # Safety
            /// Calls into the Objective-C runtime.
            pub unsafe fn barrierAfterQueueStages(
                &self,
                afterQueueStages: MTLStages,
                beforeStages: MTLStages,
            ) {
                send2::<MTLStages, MTLStages, ()>(
                    self.as_raw(),
                    sel!("barrierAfterQueueStages:beforeStages:"),
                    afterQueueStages,
                    beforeStages,
                );
            }
        }
    };
}

encoder_base_methods!(MTLRenderCommandEncoder);
encoder_base_methods!(MTLComputeCommandEncoder);
encoder_base_methods!(MTLBlitCommandEncoder);
encoder_base_methods!(MTLParallelRenderCommandEncoder);

// ---------------------------------------------------------------------------
// MTL::RenderCommandEncoder
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::RenderCommandEncoder` (`Metal/MTLRenderCommandEncoder.hpp`).
    pub struct MTLRenderCommandEncoder : "MTLRenderCommandEncoder";
);

impl MTLRenderCommandEncoder {
    // -- pipeline & resources ------------------------------------------------

    /// `-setRenderPipelineState:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRenderPipelineState(&self, pipelineState: Ref<MTLRenderPipelineState>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setRenderPipelineState:"),
            pipelineState.as_raw(),
        );
    }

    /// `-setVertexBuffer:offset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexBuffer(
        &self,
        buffer: Ref<MTLBuffer>,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVertexBuffer:offset:atIndex:"),
            buffer.as_raw(),
            offset,
            index,
        );
    }

    /// `-setVertexBufferOffset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexBufferOffset(&self, offset: NSUInteger, index: NSUInteger) {
        send2::<NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVertexBufferOffset:atIndex:"),
            offset,
            index,
        );
    }

    /// `-setVertexBytes:length:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexBytes(
        &self,
        bytes: *const c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<*const c_void, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVertexBytes:length:atIndex:"),
            bytes,
            length,
            index,
        );
    }

    /// `-setVertexBuffers:offsets:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexBuffers(&self, buffers: &[Raw], offsets: &[NSUInteger], range: NSRange) {
        send3::<*const Raw, *const NSUInteger, NSRange, ()>(
            self.as_raw(),
            sel!("setVertexBuffers:offsets:withRange:"),
            buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    /// `-setVertexTexture:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexTexture(&self, texture: Ref<MTLTexture>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVertexTexture:atIndex:"),
            texture.as_raw(),
            index,
        );
    }

    /// `-setVertexTextures:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexTextures(&self, textures: &[Raw], range: NSRange) {
        send2::<*const Raw, NSRange, ()>(
            self.as_raw(),
            sel!("setVertexTextures:withRange:"),
            textures.as_ptr(),
            range,
        );
    }

    /// `-setVertexSamplerState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexSamplerState(&self, sampler: Ref<MTLSamplerState>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVertexSamplerState:atIndex:"),
            sampler.as_raw(),
            index,
        );
    }

    /// `-setFragmentBuffer:offset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentBuffer(
        &self,
        buffer: Ref<MTLBuffer>,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setFragmentBuffer:offset:atIndex:"),
            buffer.as_raw(),
            offset,
            index,
        );
    }

    /// `-setFragmentBufferOffset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentBufferOffset(&self, offset: NSUInteger, index: NSUInteger) {
        send2::<NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setFragmentBufferOffset:atIndex:"),
            offset,
            index,
        );
    }

    /// `-setFragmentBytes:length:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentBytes(
        &self,
        bytes: *const c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<*const c_void, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setFragmentBytes:length:atIndex:"),
            bytes,
            length,
            index,
        );
    }

    /// `-setFragmentBuffers:offsets:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentBuffers(
        &self,
        buffers: &[Raw],
        offsets: &[NSUInteger],
        range: NSRange,
    ) {
        send3::<*const Raw, *const NSUInteger, NSRange, ()>(
            self.as_raw(),
            sel!("setFragmentBuffers:offsets:withRange:"),
            buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    /// `-setFragmentTexture:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentTexture(&self, texture: Ref<MTLTexture>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setFragmentTexture:atIndex:"),
            texture.as_raw(),
            index,
        );
    }

    /// `-setFragmentTextures:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentTextures(&self, textures: &[Raw], range: NSRange) {
        send2::<*const Raw, NSRange, ()>(
            self.as_raw(),
            sel!("setFragmentTextures:withRange:"),
            textures.as_ptr(),
            range,
        );
    }

    /// `-setFragmentSamplerState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentSamplerState(&self, sampler: Ref<MTLSamplerState>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setFragmentSamplerState:atIndex:"),
            sampler.as_raw(),
            index,
        );
    }

    // -- rasterizer state ------------------------------------------------------

    /// `-setViewport:` (`MTL::Viewport` passed by value).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setViewport(&self, viewport: MTLViewport) {
        send1::<MTLViewport, ()>(self.as_raw(), sel!("setViewport:"), viewport);
    }

    /// `-setViewports:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setViewports(&self, viewports: &[MTLViewport], count: NSUInteger) {
        send2::<*const MTLViewport, NSUInteger, ()>(
            self.as_raw(),
            sel!("setViewports:count:"),
            viewports.as_ptr(),
            count,
        );
    }

    /// `-setScissorRect:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setScissorRect(&self, rect: MTLScissorRect) {
        send1::<MTLScissorRect, ()>(self.as_raw(), sel!("setScissorRect:"), rect);
    }

    /// `-setScissorRects:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setScissorRects(&self, scissorRects: &[MTLScissorRect], count: NSUInteger) {
        send2::<*const MTLScissorRect, NSUInteger, ()>(
            self.as_raw(),
            sel!("setScissorRects:count:"),
            scissorRects.as_ptr(),
            count,
        );
    }

    /// `-setCullMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCullMode(&self, cullMode: MTLCullMode) {
        send1::<MTLCullMode, ()>(self.as_raw(), sel!("setCullMode:"), cullMode);
    }

    /// `-setFrontFacingWinding:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFrontFacingWinding(&self, frontFacingWinding: MTLWinding) {
        send1::<MTLWinding, ()>(
            self.as_raw(),
            sel!("setFrontFacingWinding:"),
            frontFacingWinding,
        );
    }

    /// `-setTriangleFillMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTriangleFillMode(&self, fillMode: MTLTriangleFillMode) {
        send1::<MTLTriangleFillMode, ()>(self.as_raw(), sel!("setTriangleFillMode:"), fillMode);
    }

    /// `-setDepthStencilState:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthStencilState(&self, depthStencilState: Ref<MTLDepthStencilState>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setDepthStencilState:"),
            depthStencilState.as_raw(),
        );
    }

    /// `-setDepthBias:slopeScale:clamp:` (three `float` arguments).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthBias(&self, depthBias: f32, slopeScale: f32, clamp: f32) {
        send3::<f32, f32, f32, ()>(
            self.as_raw(),
            sel!("setDepthBias:slopeScale:clamp:"),
            depthBias,
            slopeScale,
            clamp,
        );
    }

    /// `-setDepthClipMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthClipMode(&self, depthClipMode: MTLDepthClipMode) {
        send1::<MTLDepthClipMode, ()>(self.as_raw(), sel!("setDepthClipMode:"), depthClipMode);
    }

    /// `-setBlendColorRed:green:blue:alpha:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBlendColor(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        send4::<f32, f32, f32, f32, ()>(
            self.as_raw(),
            sel!("setBlendColorRed:green:blue:alpha:"),
            red,
            green,
            blue,
            alpha,
        );
    }

    /// `-setStencilReferenceValue:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilReferenceValue(&self, referenceValue: u32) {
        send1::<u32, ()>(
            self.as_raw(),
            sel!("setStencilReferenceValue:"),
            referenceValue,
        );
    }

    /// `-setDepthTestMinBound:maxBound:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthTestBounds(&self, minBound: f32, maxBound: f32) {
        send2::<f32, f32, ()>(
            self.as_raw(),
            sel!("setDepthTestMinBound:maxBound:"),
            minBound,
            maxBound,
        );
    }

    /// `-setVisibilityResultMode:offset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVisibilityResultMode(
        &self,
        mode: MTLVisibilityResultMode,
        offset: NSUInteger,
    ) {
        send2::<MTLVisibilityResultMode, NSUInteger, ()>(
            self.as_raw(),
            sel!("setVisibilityResultMode:offset:"),
            mode,
            offset,
        );
    }

    /// `-setTessellationFactorBuffer:offset:instanceStride:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTessellationFactorBuffer(
        &self,
        buffer: Ref<MTLBuffer>,
        offset: NSUInteger,
        instanceStride: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setTessellationFactorBuffer:offset:instanceStride:"),
            buffer.as_raw(),
            offset,
            instanceStride,
        );
    }

    /// `-setTessellationFactorScale:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTessellationFactorScale(&self, scale: f32) {
        send1::<f32, ()>(self.as_raw(), sel!("setTessellationFactorScale:"), scale);
    }

    /// `-setThreadgroupMemoryLength:offset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setThreadgroupMemoryLength(
        &self,
        length: NSUInteger,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<NSUInteger, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setThreadgroupMemoryLength:offset:atIndex:"),
            length,
            offset,
            index,
        );
    }

    // -- draws -----------------------------------------------------------------

    /// `-drawPrimitives:vertexStart:vertexCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawPrimitives(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
    ) {
        send3::<MTLPrimitiveType, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("drawPrimitives:vertexStart:vertexCount:"),
            primitiveType,
            vertexStart,
            vertexCount,
        );
    }

    /// `-drawPrimitives:vertexStart:vertexCount:instanceCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawPrimitivesInstanced(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
    ) {
        send4::<MTLPrimitiveType, NSUInteger, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("drawPrimitives:vertexStart:vertexCount:instanceCount:"),
            primitiveType,
            vertexStart,
            vertexCount,
            instanceCount,
        );
    }

    /// `-drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawPrimitivesInstancedBaseInstance(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) {
        send5::<MTLPrimitiveType, NSUInteger, NSUInteger, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:"),
            primitiveType,
            vertexStart,
            vertexCount,
            instanceCount,
            baseInstance,
        );
    }

    /// `-drawPrimitives:indirectBuffer:indirectBufferOffset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawPrimitivesIndirect(
        &self,
        primitiveType: MTLPrimitiveType,
        indirectBuffer: Ref<MTLBuffer>,
        indirectBufferOffset: NSUInteger,
    ) {
        send3::<MTLPrimitiveType, Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("drawPrimitives:indirectBuffer:indirectBufferOffset:"),
            primitiveType,
            indirectBuffer.as_raw(),
            indirectBufferOffset,
        );
    }

    /// `-drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawIndexedPrimitives(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: Ref<MTLBuffer>,
        indexBufferOffset: NSUInteger,
    ) {
        send5::<MTLPrimitiveType, NSUInteger, MTLIndexType, Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:"),
            primitiveType,
            indexCount,
            indexType,
            indexBuffer.as_raw(),
            indexBufferOffset,
        );
    }

    /// `-drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawIndexedPrimitivesInstanced(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: Ref<MTLBuffer>,
        indexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
    ) {
        send6::<MTLPrimitiveType, NSUInteger, MTLIndexType, Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!(
                "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:"
            ),
            primitiveType,
            indexCount,
            indexType,
            indexBuffer.as_raw(),
            indexBufferOffset,
            instanceCount,
        );
    }

    /// `-drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawIndexedPrimitivesInstancedBaseVertexBaseInstance(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: Ref<MTLBuffer>,
        indexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseVertex: NSUInteger,
        baseInstance: NSUInteger,
    ) {
        send8::<
            MTLPrimitiveType,
            NSUInteger,
            MTLIndexType,
            Raw,
            NSUInteger,
            NSUInteger,
            NSUInteger,
            NSUInteger,
            (),
        >(
            self.as_raw(),
            sel!(
                "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:"
            ),
            primitiveType,
            indexCount,
            indexType,
            indexBuffer.as_raw(),
            indexBufferOffset,
            instanceCount,
            baseVertex,
            baseInstance,
        );
    }

    /// `-drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawPatches(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: Ref<MTLBuffer>,
        patchIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) {
        send7::<NSUInteger, NSUInteger, NSUInteger, Raw, NSUInteger, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!(
                "drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:"
            ),
            numberOfPatchControlPoints,
            patchStart,
            patchCount,
            patchIndexBuffer.as_raw(),
            patchIndexBufferOffset,
            instanceCount,
            baseInstance,
        );
    }

    /// `-drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drawIndexedPatches(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: Ref<MTLBuffer>,
        patchIndexBufferOffset: NSUInteger,
        controlPointIndexBuffer: Ref<MTLBuffer>,
        controlPointIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) {
        send9::<
            NSUInteger,
            NSUInteger,
            NSUInteger,
            Raw,
            NSUInteger,
            Raw,
            NSUInteger,
            NSUInteger,
            NSUInteger,
            (),
        >(
            self.as_raw(),
            sel!(
                "drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:"
            ),
            numberOfPatchControlPoints,
            patchStart,
            patchCount,
            patchIndexBuffer.as_raw(),
            patchIndexBufferOffset,
            controlPointIndexBuffer.as_raw(),
            controlPointIndexBufferOffset,
            instanceCount,
            baseInstance,
        );
    }

    /// `-executeCommandsInBuffer:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn executeCommandsInBuffer(
        &self,
        indirectCommandBuffer: Raw,
        executionRange: NSRange,
    ) {
        send2::<Raw, NSRange, ()>(
            self.as_raw(),
            sel!("executeCommandsInBuffer:withRange:"),
            indirectCommandBuffer,
            executionRange,
        );
    }

    // -- fences / barriers / residency -----------------------------------------

    /// `-updateFence:afterStages:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn updateFence(&self, fence: Ref<MTLFence>, stages: MTLRenderStages) {
        send2::<Raw, MTLRenderStages, ()>(
            self.as_raw(),
            sel!("updateFence:afterStages:"),
            fence.as_raw(),
            stages,
        );
    }

    /// `-waitForFence:beforeStages:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitForFence(&self, fence: Ref<MTLFence>, stages: MTLRenderStages) {
        send2::<Raw, MTLRenderStages, ()>(
            self.as_raw(),
            sel!("waitForFence:beforeStages:"),
            fence.as_raw(),
            stages,
        );
    }

    /// `-memoryBarrierWithScope:afterStages:beforeStages:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn memoryBarrier(
        &self,
        scope: MTLBarrierScope,
        after: MTLRenderStages,
        before: MTLRenderStages,
    ) {
        send3::<MTLBarrierScope, MTLRenderStages, MTLRenderStages, ()>(
            self.as_raw(),
            sel!("memoryBarrierWithScope:afterStages:beforeStages:"),
            scope,
            after,
            before,
        );
    }

    /// `-textureBarrier`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn textureBarrier(&self) {
        send::<()>(self.as_raw(), sel!("textureBarrier"));
    }

    /// `-useResource:usage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useResource(&self, resource: Ref<MTLResource>, usage: MTLResourceUsage) {
        send2::<Raw, MTLResourceUsage, ()>(
            self.as_raw(),
            sel!("useResource:usage:"),
            resource.as_raw(),
            usage,
        );
    }

    /// `-useResources:count:usage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useResources(
        &self,
        resources: &[Raw],
        count: NSUInteger,
        usage: MTLResourceUsage,
    ) {
        send3::<*const Raw, NSUInteger, MTLResourceUsage, ()>(
            self.as_raw(),
            sel!("useResources:count:usage:"),
            resources.as_ptr(),
            count,
            usage,
        );
    }

    /// `-useHeap:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useHeap(&self, heap: Ref<MTLHeap>) {
        send1::<Raw, ()>(self.as_raw(), sel!("useHeap:"), heap.as_raw());
    }

    /// `-useHeaps:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useHeaps(&self, heaps: &[Raw], count: NSUInteger) {
        send2::<*const Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("useHeaps:count:"),
            heaps.as_ptr(),
            count,
        );
    }

    // -- pass-level tweaks ------------------------------------------------------

    /// `-setColorStoreAction:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setColorStoreAction(
        &self,
        storeAction: MTLStoreAction,
        colorAttachmentIndex: NSUInteger,
    ) {
        send2::<MTLStoreAction, NSUInteger, ()>(
            self.as_raw(),
            sel!("setColorStoreAction:atIndex:"),
            storeAction,
            colorAttachmentIndex,
        );
    }

    /// `-setColorStoreActionOptions:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setColorStoreActionOptions(
        &self,
        storeActionOptions: MTLStoreActionOptions,
        colorAttachmentIndex: NSUInteger,
    ) {
        send2::<MTLStoreActionOptions, NSUInteger, ()>(
            self.as_raw(),
            sel!("setColorStoreActionOptions:atIndex:"),
            storeActionOptions,
            colorAttachmentIndex,
        );
    }

    /// `-setDepthStoreAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthStoreAction(&self, storeAction: MTLStoreAction) {
        send1::<MTLStoreAction, ()>(self.as_raw(), sel!("setDepthStoreAction:"), storeAction);
    }

    /// `-setStencilStoreAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilStoreAction(&self, storeAction: MTLStoreAction) {
        send1::<MTLStoreAction, ()>(self.as_raw(), sel!("setStencilStoreAction:"), storeAction);
    }

    /// `-tileWidth`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn tileWidth(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("tileWidth"))
    }

    /// `-tileHeight`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn tileHeight(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("tileHeight"))
    }
}

// ---------------------------------------------------------------------------
// MTL::ComputeCommandEncoder
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::ComputeCommandEncoder` (`Metal/MTLComputeCommandEncoder.hpp`).
    pub struct MTLComputeCommandEncoder : "MTLComputeCommandEncoder";
);

impl MTLComputeCommandEncoder {
    /// `-setComputePipelineState:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setComputePipelineState(&self, state: Ref<MTLComputePipelineState>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setComputePipelineState:"),
            state.as_raw(),
        );
    }

    /// `-setBuffer:offset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBuffer(&self, buffer: Ref<MTLBuffer>, offset: NSUInteger, index: NSUInteger) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setBuffer:offset:atIndex:"),
            buffer.as_raw(),
            offset,
            index,
        );
    }

    /// `-setBufferOffset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBufferOffset(&self, offset: NSUInteger, index: NSUInteger) {
        send2::<NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setBufferOffset:atIndex:"),
            offset,
            index,
        );
    }

    /// `-setBuffers:offsets:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBuffers(&self, buffers: &[Raw], offsets: &[NSUInteger], range: NSRange) {
        send3::<*const Raw, *const NSUInteger, NSRange, ()>(
            self.as_raw(),
            sel!("setBuffers:offsets:withRange:"),
            buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    /// `-setBytes:length:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBytes(&self, bytes: *const c_void, length: NSUInteger, index: NSUInteger) {
        send3::<*const c_void, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setBytes:length:atIndex:"),
            bytes,
            length,
            index,
        );
    }

    /// `-setThreadgroupMemoryLength:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setThreadgroupMemoryLength(&self, length: NSUInteger, index: NSUInteger) {
        send2::<NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setThreadgroupMemoryLength:atIndex:"),
            length,
            index,
        );
    }

    /// `-setTexture:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTexture(&self, texture: Ref<MTLTexture>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setTexture:atIndex:"),
            texture.as_raw(),
            index,
        );
    }

    /// `-setTextures:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTextures(&self, textures: &[Raw], range: NSRange) {
        send2::<*const Raw, NSRange, ()>(
            self.as_raw(),
            sel!("setTextures:withRange:"),
            textures.as_ptr(),
            range,
        );
    }

    /// `-setSamplerState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSamplerState(&self, sampler: Ref<MTLSamplerState>, index: NSUInteger) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setSamplerState:atIndex:"),
            sampler.as_raw(),
            index,
        );
    }

    /// `-dispatchThreadgroups:threadsPerThreadgroup:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dispatchThreadgroups(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) {
        send2::<MTLSize, MTLSize, ()>(
            self.as_raw(),
            sel!("dispatchThreadgroups:threadsPerThreadgroup:"),
            threadgroupsPerGrid,
            threadsPerThreadgroup,
        );
    }

    /// `-dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dispatchThreadgroupsIndirect(
        &self,
        indirectBuffer: Ref<MTLBuffer>,
        indirectBufferOffset: NSUInteger,
        threadsPerThreadgroup: MTLSize,
    ) {
        send3::<Raw, NSUInteger, MTLSize, ()>(
            self.as_raw(),
            sel!(
                "dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:"
            ),
            indirectBuffer.as_raw(),
            indirectBufferOffset,
            threadsPerThreadgroup,
        );
    }

    /// `-dispatchThreads:threadsPerThreadgroup:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dispatchThreads(&self, threadsPerGrid: MTLSize, threadsPerThreadgroup: MTLSize) {
        send2::<MTLSize, MTLSize, ()>(
            self.as_raw(),
            sel!("dispatchThreads:threadsPerThreadgroup:"),
            threadsPerGrid,
            threadsPerThreadgroup,
        );
    }

    /// `-setStageInRegion:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStageInRegion(&self, region: crate::metal::types::MTLRegion) {
        send1::<crate::metal::types::MTLRegion, ()>(
            self.as_raw(),
            sel!("setStageInRegion:"),
            region,
        );
    }

    /// `-setStageInRegionWithIndirectBuffer:indirectBufferOffset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStageInRegionIndirect(
        &self,
        indirectBuffer: Ref<MTLBuffer>,
        indirectBufferOffset: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setStageInRegionWithIndirectBuffer:indirectBufferOffset:"),
            indirectBuffer.as_raw(),
            indirectBufferOffset,
        );
    }

    /// `-dispatchType`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dispatchType(&self) -> MTLDispatchType {
        send::<MTLDispatchType>(self.as_raw(), sel!("dispatchType"))
    }

    /// `-executeCommandsInBuffer:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn executeCommandsInBuffer(
        &self,
        indirectCommandBuffer: Raw,
        executionRange: NSRange,
    ) {
        send2::<Raw, NSRange, ()>(
            self.as_raw(),
            sel!("executeCommandsInBuffer:withRange:"),
            indirectCommandBuffer,
            executionRange,
        );
    }

    /// `-memoryBarrierWithScope:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn memoryBarrier(&self, scope: MTLBarrierScope) {
        send1::<MTLBarrierScope, ()>(self.as_raw(), sel!("memoryBarrierWithScope:"), scope);
    }

    /// `-memoryBarrierWithResources:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn memoryBarrierWithResources(&self, resources: &[Raw], count: NSUInteger) {
        send2::<*const Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("memoryBarrierWithResources:count:"),
            resources.as_ptr(),
            count,
        );
    }

    /// `-updateFence:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn updateFence(&self, fence: Ref<MTLFence>) {
        send1::<Raw, ()>(self.as_raw(), sel!("updateFence:"), fence.as_raw());
    }

    /// `-waitForFence:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitForFence(&self, fence: Ref<MTLFence>) {
        send1::<Raw, ()>(self.as_raw(), sel!("waitForFence:"), fence.as_raw());
    }

    /// `-useResource:usage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useResource(&self, resource: Ref<MTLResource>, usage: MTLResourceUsage) {
        send2::<Raw, MTLResourceUsage, ()>(
            self.as_raw(),
            sel!("useResource:usage:"),
            resource.as_raw(),
            usage,
        );
    }

    /// `-useResources:count:usage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useResources(
        &self,
        resources: &[Raw],
        count: NSUInteger,
        usage: MTLResourceUsage,
    ) {
        send3::<*const Raw, NSUInteger, MTLResourceUsage, ()>(
            self.as_raw(),
            sel!("useResources:count:usage:"),
            resources.as_ptr(),
            count,
            usage,
        );
    }

    /// `-useHeap:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useHeap(&self, heap: Ref<MTLHeap>) {
        send1::<Raw, ()>(self.as_raw(), sel!("useHeap:"), heap.as_raw());
    }

    /// `-useHeaps:count:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn useHeaps(&self, heaps: &[Raw], count: NSUInteger) {
        send2::<*const Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("useHeaps:count:"),
            heaps.as_ptr(),
            count,
        );
    }
}

// ---------------------------------------------------------------------------
// MTL::BlitCommandEncoder
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::BlitCommandEncoder` (`Metal/MTLBlitCommandEncoder.hpp`).
    pub struct MTLBlitCommandEncoder : "MTLBlitCommandEncoder";
);

impl MTLBlitCommandEncoder {
    /// `-copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromBufferToBuffer(
        &self,
        sourceBuffer: Ref<MTLBuffer>,
        sourceOffset: NSUInteger,
        destinationBuffer: Ref<MTLBuffer>,
        destinationOffset: NSUInteger,
        size: NSUInteger,
    ) {
        send5::<Raw, NSUInteger, Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:"),
            sourceBuffer.as_raw(),
            sourceOffset,
            destinationBuffer.as_raw(),
            destinationOffset,
            size,
        );
    }

    /// `-copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromBufferToTexture(
        &self,
        sourceBuffer: Ref<MTLBuffer>,
        sourceOffset: NSUInteger,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        sourceSize: MTLSize,
        destinationTexture: Ref<MTLTexture>,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: crate::metal::types::MTLOrigin,
    ) {
        send9::<
            Raw,
            NSUInteger,
            NSUInteger,
            NSUInteger,
            MTLSize,
            Raw,
            NSUInteger,
            NSUInteger,
            crate::metal::types::MTLOrigin,
            (),
        >(
            self.as_raw(),
            sel!(
                "copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"
            ),
            sourceBuffer.as_raw(),
            sourceOffset,
            sourceBytesPerRow,
            sourceBytesPerImage,
            sourceSize,
            destinationTexture.as_raw(),
            destinationSlice,
            destinationLevel,
            destinationOrigin,
        );
    }

    /// `-copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromTextureToTexture(
        &self,
        sourceTexture: Ref<MTLTexture>,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: crate::metal::types::MTLOrigin,
        sourceSize: MTLSize,
        destinationTexture: Ref<MTLTexture>,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: crate::metal::types::MTLOrigin,
    ) {
        send9::<
            Raw,
            NSUInteger,
            NSUInteger,
            crate::metal::types::MTLOrigin,
            MTLSize,
            Raw,
            NSUInteger,
            NSUInteger,
            crate::metal::types::MTLOrigin,
            (),
        >(
            self.as_raw(),
            sel!(
                "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"
            ),
            sourceTexture.as_raw(),
            sourceSlice,
            sourceLevel,
            sourceOrigin,
            sourceSize,
            destinationTexture.as_raw(),
            destinationSlice,
            destinationLevel,
            destinationOrigin,
        );
    }

    /// `-copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromTextureToBuffer(
        &self,
        sourceTexture: Ref<MTLTexture>,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: crate::metal::types::MTLOrigin,
        sourceSize: MTLSize,
        destinationBuffer: Ref<MTLBuffer>,
        destinationOffset: NSUInteger,
        destinationBytesPerRow: NSUInteger,
        destinationBytesPerImage: NSUInteger,
    ) {
        send9::<
            Raw,
            NSUInteger,
            NSUInteger,
            crate::metal::types::MTLOrigin,
            MTLSize,
            Raw,
            NSUInteger,
            NSUInteger,
            NSUInteger,
            (),
        >(
            self.as_raw(),
            sel!(
                "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:"
            ),
            sourceTexture.as_raw(),
            sourceSlice,
            sourceLevel,
            sourceOrigin,
            sourceSize,
            destinationBuffer.as_raw(),
            destinationOffset,
            destinationBytesPerRow,
            destinationBytesPerImage,
        );
    }

    /// `-copyFromTexture:toTexture:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromTexture(
        &self,
        sourceTexture: Ref<MTLTexture>,
        destinationTexture: Ref<MTLTexture>,
    ) {
        send2::<Raw, Raw, ()>(
            self.as_raw(),
            sel!("copyFromTexture:toTexture:"),
            sourceTexture.as_raw(),
            destinationTexture.as_raw(),
        );
    }

    /// `-copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn copyFromTextureRange(
        &self,
        sourceTexture: Ref<MTLTexture>,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        destinationTexture: Ref<MTLTexture>,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        sliceCount: NSUInteger,
        levelCount: NSUInteger,
    ) {
        send8::<Raw, NSUInteger, NSUInteger, Raw, NSUInteger, NSUInteger, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!(
                "copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:"
            ),
            sourceTexture.as_raw(),
            sourceSlice,
            sourceLevel,
            destinationTexture.as_raw(),
            destinationSlice,
            destinationLevel,
            sliceCount,
            levelCount,
        );
    }

    /// `-generateMipmapsForTexture:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn generateMipmaps(&self, texture: Ref<MTLTexture>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("generateMipmapsForTexture:"),
            texture.as_raw(),
        );
    }

    /// `-synchronizeResource:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn synchronizeResource(&self, resource: Ref<MTLResource>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("synchronizeResource:"),
            resource.as_raw(),
        );
    }

    /// `-synchronizeTexture:slice:level:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn synchronizeTexture(
        &self,
        texture: Ref<MTLTexture>,
        slice: NSUInteger,
        level: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("synchronizeTexture:slice:level:"),
            texture.as_raw(),
            slice,
            level,
        );
    }

    /// `-fillBuffer:range:value:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fillBuffer(&self, buffer: Ref<MTLBuffer>, range: NSRange, value: u8) {
        send3::<Raw, NSRange, u8, ()>(
            self.as_raw(),
            sel!("fillBuffer:range:value:"),
            buffer.as_raw(),
            range,
            value,
        );
    }

    /// `-optimizeContentsForGPUAccess:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn optimizeContentsForGPUAccess(&self, texture: Ref<MTLTexture>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("optimizeContentsForGPUAccess:"),
            texture.as_raw(),
        );
    }

    /// `-optimizeContentsForCPUAccess:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn optimizeContentsForCPUAccess(&self, texture: Ref<MTLTexture>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("optimizeContentsForCPUAccess:"),
            texture.as_raw(),
        );
    }

    /// `-updateFence:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn updateFence(&self, fence: Ref<MTLFence>) {
        send1::<Raw, ()>(self.as_raw(), sel!("updateFence:"), fence.as_raw());
    }

    /// `-waitForFence:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn waitForFence(&self, fence: Ref<MTLFence>) {
        send1::<Raw, ()>(self.as_raw(), sel!("waitForFence:"), fence.as_raw());
    }
}

// ---------------------------------------------------------------------------
// MTL::ParallelRenderCommandEncoder
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::ParallelRenderCommandEncoder`
    /// (`Metal/MTLParallelRenderCommandEncoder.hpp`).
    pub struct MTLParallelRenderCommandEncoder : "MTLParallelRenderCommandEncoder";
);

impl MTLParallelRenderCommandEncoder {
    /// `-renderCommandEncoder` (per-thread command encoder; +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn renderCommandEncoder(&self) -> Strong<MTLRenderCommandEncoder> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("renderCommandEncoder")))
    }

    /// `-setColorStoreAction:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setColorStoreAction(
        &self,
        storeAction: MTLStoreAction,
        colorAttachmentIndex: NSUInteger,
    ) {
        send2::<MTLStoreAction, NSUInteger, ()>(
            self.as_raw(),
            sel!("setColorStoreAction:atIndex:"),
            storeAction,
            colorAttachmentIndex,
        );
    }

    /// `-setColorStoreActionOptions:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setColorStoreActionOptions(
        &self,
        storeActionOptions: MTLStoreActionOptions,
        colorAttachmentIndex: NSUInteger,
    ) {
        send2::<MTLStoreActionOptions, NSUInteger, ()>(
            self.as_raw(),
            sel!("setColorStoreActionOptions:atIndex:"),
            storeActionOptions,
            colorAttachmentIndex,
        );
    }

    /// `-setDepthStoreAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthStoreAction(&self, storeAction: MTLStoreAction) {
        send1::<MTLStoreAction, ()>(self.as_raw(), sel!("setDepthStoreAction:"), storeAction);
    }

    /// `-setStencilStoreAction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilStoreAction(&self, storeAction: MTLStoreAction) {
        send1::<MTLStoreAction, ()>(self.as_raw(), sel!("setStencilStoreAction:"), storeAction);
    }
}
