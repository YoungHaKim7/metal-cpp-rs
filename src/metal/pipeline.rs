//! Pipeline state & descriptors — `MTL::RenderPipelineDescriptor`,
//! `MTL::RenderPipelineState`, `MTL::ComputePipelineDescriptor`,
//! `MTL::ComputePipelineState`, `MTL::DepthStencilDescriptor`,
//! `MTL::StencilDescriptor`, `MTL::DepthStencilState`, the
//! `MTL::VertexDescriptor` family and `MTL::BinaryArchive`
//! (`Metal/MTLRenderPipeline.hpp`, `MTLComputePipeline.hpp`,
//! `MTLDepthStencil.hpp`, `MTLVertexDescriptor.hpp`, `MTLBinaryArchive.hpp`).

use crate::foundation::{NSError, NSString, NSURL};
use crate::metal::device::MTLDevice;
use crate::metal::enums::{
    MTLBlendFactor, MTLBlendOperation, MTLColorWriteMask, MTLCompareFunction,
    MTLPrimitiveTopologyClass, MTLShaderValidation, MTLStencilOperation, MTLVertexFormat,
    MTLVertexStepFunction,
};
use crate::metal::shader::MTLFunction;
use crate::metal::types::{MTLResourceID, MTLSize};
use crate::objc_abi::{
    bool_arg, send, send1, send2, send_bool, send_stret, NSUInteger, Obj, Raw, Ref, Strong,
};
use crate::objc_abi::{objc_class, sel};

// ---------------------------------------------------------------------------
// MTL::RenderPipelineDescriptor
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::RenderPipelineDescriptor` (`Metal/MTLRenderPipeline.hpp`).
    pub struct MTLRenderPipelineDescriptor : "MTLRenderPipelineDescriptor";
);

impl MTLRenderPipelineDescriptor {
    /// `-reset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn reset(&self) {
        send::<()>(self.as_raw(), sel!("reset"));
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

    /// `-vertexFunction` / `-setVertexFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn vertexFunction(&self) -> Ref<MTLFunction> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("vertexFunction")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexFunction(&self, vertexFunction: Ref<MTLFunction>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setVertexFunction:"), vertexFunction.as_raw());
    }

    /// `-fragmentFunction` / `-setFragmentFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fragmentFunction(&self) -> Ref<MTLFunction> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("fragmentFunction")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFragmentFunction(&self, fragmentFunction: Ref<MTLFunction>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setFragmentFunction:"), fragmentFunction.as_raw());
    }

    /// `-vertexDescriptor` / `-setVertexDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn vertexDescriptor(&self) -> Ref<MTLVertexDescriptor> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("vertexDescriptor")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setVertexDescriptor(&self, vertexDescriptor: Ref<MTLVertexDescriptor>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setVertexDescriptor:"), vertexDescriptor.as_raw());
    }

    /// `-colorAttachments`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn colorAttachments(&self) -> Ref<MTLRenderPipelineColorAttachmentDescriptorArray> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("colorAttachments")))
    }

    /// `-sampleCount` / `-setSampleCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sampleCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("sampleCount"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSampleCount(&self, sampleCount: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setSampleCount:"), sampleCount);
    }

    /// `-rasterSampleCount` / `-setRasterSampleCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn rasterSampleCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("rasterSampleCount"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRasterSampleCount(&self, rasterSampleCount: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setRasterSampleCount:"), rasterSampleCount);
    }

    /// `-depthAttachmentPixelFormat` / `-setDepthAttachmentPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthAttachmentPixelFormat(&self) -> crate::metal::enums::MTLPixelFormat {
        send::<crate::metal::enums::MTLPixelFormat>(self.as_raw(), sel!("depthAttachmentPixelFormat"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthAttachmentPixelFormat(
        &self,
        depthAttachmentPixelFormat: crate::metal::enums::MTLPixelFormat,
    ) {
        send1::<crate::metal::enums::MTLPixelFormat, ()>(
            self.as_raw(),
            sel!("setDepthAttachmentPixelFormat:"),
            depthAttachmentPixelFormat,
        );
    }

    /// `-stencilAttachmentPixelFormat` / `-setStencilAttachmentPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stencilAttachmentPixelFormat(&self) -> crate::metal::enums::MTLPixelFormat {
        send::<crate::metal::enums::MTLPixelFormat>(self.as_raw(), sel!("stencilAttachmentPixelFormat"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilAttachmentPixelFormat(
        &self,
        stencilAttachmentPixelFormat: crate::metal::enums::MTLPixelFormat,
    ) {
        send1::<crate::metal::enums::MTLPixelFormat, ()>(
            self.as_raw(),
            sel!("setStencilAttachmentPixelFormat:"),
            stencilAttachmentPixelFormat,
        );
    }

    /// `-inputPrimitiveTopology` / `-setInputPrimitiveTopology:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn inputPrimitiveTopology(&self) -> MTLPrimitiveTopologyClass {
        send::<MTLPrimitiveTopologyClass>(self.as_raw(), sel!("inputPrimitiveTopology"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setInputPrimitiveTopology(&self, inputPrimitiveTopology: MTLPrimitiveTopologyClass) {
        send1::<MTLPrimitiveTopologyClass, ()>(
            self.as_raw(),
            sel!("setInputPrimitiveTopology:"),
            inputPrimitiveTopology,
        );
    }

    /// `-isAlphaToCoverageEnabled` / `-setAlphaToCoverageEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isAlphaToCoverageEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isAlphaToCoverageEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAlphaToCoverageEnabled(&self, alphaToCoverageEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setAlphaToCoverageEnabled:"),
            bool_arg(alphaToCoverageEnabled),
        );
    }

    /// `-isAlphaToOneEnabled` / `-setAlphaToOneEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isAlphaToOneEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isAlphaToOneEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAlphaToOneEnabled(&self, alphaToOneEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setAlphaToOneEnabled:"),
            bool_arg(alphaToOneEnabled),
        );
    }

    /// `-isRasterizationEnabled` / `-setRasterizationEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isRasterizationEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isRasterizationEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRasterizationEnabled(&self, rasterizationEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setRasterizationEnabled:"),
            bool_arg(rasterizationEnabled),
        );
    }

    /// `-shaderValidation` / `-setShaderValidation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn shaderValidation(&self) -> MTLShaderValidation {
        send::<MTLShaderValidation>(self.as_raw(), sel!("shaderValidation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setShaderValidation(&self, shaderValidation: MTLShaderValidation) {
        send1::<MTLShaderValidation, ()>(self.as_raw(), sel!("setShaderValidation:"), shaderValidation);
    }
}

// ---------------------------------------------------------------------------
// MTL::RenderPipelineColorAttachmentDescriptor(Array)
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::RenderPipelineColorAttachmentDescriptorArray`.
    pub struct MTLRenderPipelineColorAttachmentDescriptorArray : "MTLRenderPipelineColorAttachmentDescriptorArray";
);

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    /// `-objectAtIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectAtIndexedSubscript(
        &self,
        attachmentIndex: NSUInteger,
    ) -> Ref<MTLRenderPipelineColorAttachmentDescriptor> {
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
        attachment: Ref<MTLRenderPipelineColorAttachmentDescriptor>,
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
    /// `MTL::RenderPipelineColorAttachmentDescriptor`.
    pub struct MTLRenderPipelineColorAttachmentDescriptor : "MTLRenderPipelineColorAttachmentDescriptor";
);

impl MTLRenderPipelineColorAttachmentDescriptor {
    /// `-pixelFormat` / `-setPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn pixelFormat(&self) -> crate::metal::enums::MTLPixelFormat {
        send::<crate::metal::enums::MTLPixelFormat>(self.as_raw(), sel!("pixelFormat"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPixelFormat(&self, pixelFormat: crate::metal::enums::MTLPixelFormat) {
        send1::<crate::metal::enums::MTLPixelFormat, ()>(
            self.as_raw(),
            sel!("setPixelFormat:"),
            pixelFormat,
        );
    }

    /// `-isBlendingEnabled` / `-setBlendingEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isBlendingEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isBlendingEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBlendingEnabled(&self, blendingEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setBlendingEnabled:"),
            bool_arg(blendingEnabled),
        );
    }

    /// `-sourceRGBBlendFactor` / `-setSourceRGBBlendFactor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sourceRGBBlendFactor(&self) -> MTLBlendFactor {
        send::<MTLBlendFactor>(self.as_raw(), sel!("sourceRGBBlendFactor"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSourceRGBBlendFactor(&self, sourceRGBBlendFactor: MTLBlendFactor) {
        send1::<MTLBlendFactor, ()>(
            self.as_raw(),
            sel!("setSourceRGBBlendFactor:"),
            sourceRGBBlendFactor,
        );
    }

    /// `-destinationRGBBlendFactor` / `-setDestinationRGBBlendFactor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn destinationRGBBlendFactor(&self) -> MTLBlendFactor {
        send::<MTLBlendFactor>(self.as_raw(), sel!("destinationRGBBlendFactor"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDestinationRGBBlendFactor(&self, destinationRGBBlendFactor: MTLBlendFactor) {
        send1::<MTLBlendFactor, ()>(
            self.as_raw(),
            sel!("setDestinationRGBBlendFactor:"),
            destinationRGBBlendFactor,
        );
    }

    /// `-sourceAlphaBlendFactor` / `-setSourceAlphaBlendFactor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sourceAlphaBlendFactor(&self) -> MTLBlendFactor {
        send::<MTLBlendFactor>(self.as_raw(), sel!("sourceAlphaBlendFactor"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSourceAlphaBlendFactor(&self, sourceAlphaBlendFactor: MTLBlendFactor) {
        send1::<MTLBlendFactor, ()>(
            self.as_raw(),
            sel!("setSourceAlphaBlendFactor:"),
            sourceAlphaBlendFactor,
        );
    }

    /// `-destinationAlphaBlendFactor` / `-setDestinationAlphaBlendFactor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn destinationAlphaBlendFactor(&self) -> MTLBlendFactor {
        send::<MTLBlendFactor>(self.as_raw(), sel!("destinationAlphaBlendFactor"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDestinationAlphaBlendFactor(&self, destinationAlphaBlendFactor: MTLBlendFactor) {
        send1::<MTLBlendFactor, ()>(
            self.as_raw(),
            sel!("setDestinationAlphaBlendFactor:"),
            destinationAlphaBlendFactor,
        );
    }

    /// `-rgbBlendOperation` / `-setRgbBlendOperation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn rgbBlendOperation(&self) -> MTLBlendOperation {
        send::<MTLBlendOperation>(self.as_raw(), sel!("rgbBlendOperation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRgbBlendOperation(&self, rgbBlendOperation: MTLBlendOperation) {
        send1::<MTLBlendOperation, ()>(
            self.as_raw(),
            sel!("setRgbBlendOperation:"),
            rgbBlendOperation,
        );
    }

    /// `-alphaBlendOperation` / `-setAlphaBlendOperation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn alphaBlendOperation(&self) -> MTLBlendOperation {
        send::<MTLBlendOperation>(self.as_raw(), sel!("alphaBlendOperation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAlphaBlendOperation(&self, alphaBlendOperation: MTLBlendOperation) {
        send1::<MTLBlendOperation, ()>(
            self.as_raw(),
            sel!("setAlphaBlendOperation:"),
            alphaBlendOperation,
        );
    }

    /// `-writeMask` / `-setWriteMask:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn writeMask(&self) -> MTLColorWriteMask {
        send::<MTLColorWriteMask>(self.as_raw(), sel!("writeMask"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setWriteMask(&self, writeMask: MTLColorWriteMask) {
        send1::<MTLColorWriteMask, ()>(self.as_raw(), sel!("setWriteMask:"), writeMask);
    }
}

// ---------------------------------------------------------------------------
// MTL::RenderPipelineState
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::RenderPipelineState` (`Metal/MTLRenderPipeline.hpp`).
    pub struct MTLRenderPipelineState : "MTLRenderPipelineState";
);

impl MTLRenderPipelineState {
    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-maxTotalThreadsPerThreadgroup`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxTotalThreadsPerThreadgroup"))
    }

    /// `-threadgroupSizeMatchesTileSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn threadgroupSizeMatchesTileSize(&self) -> bool {
        send_bool(self.as_raw(), sel!("threadgroupSizeMatchesTileSize"))
    }

    /// `-imageblockSampleLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn imageblockSampleLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("imageblockSampleLength"))
    }

    /// `-gpuResourceID`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn gpuResourceID(&self) -> MTLResourceID {
        send::<MTLResourceID>(self.as_raw(), sel!("gpuResourceID"))
    }

    /// `-shaderValidation`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn shaderValidation(&self) -> MTLShaderValidation {
        send::<MTLShaderValidation>(self.as_raw(), sel!("shaderValidation"))
    }

    /// `-supportIndirectCommandBuffers`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportIndirectCommandBuffers(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportIndirectCommandBuffers"))
    }
}

// ---------------------------------------------------------------------------
// MTL::ComputePipelineDescriptor / State
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::ComputePipelineDescriptor` (`Metal/MTLComputePipeline.hpp`).
    pub struct MTLComputePipelineDescriptor : "MTLComputePipelineDescriptor";
);

impl MTLComputePipelineDescriptor {
    /// `-reset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn reset(&self) {
        send::<()>(self.as_raw(), sel!("reset"));
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

    /// `-computeFunction` / `-setComputeFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn computeFunction(&self) -> Ref<MTLFunction> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("computeFunction")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setComputeFunction(&self, computeFunction: Ref<MTLFunction>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setComputeFunction:"), computeFunction.as_raw());
    }

    /// `-maxTotalThreadsPerThreadgroup` / `-setMaxTotalThreadsPerThreadgroup:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxTotalThreadsPerThreadgroup"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMaxTotalThreadsPerThreadgroup(&self, maxTotalThreadsPerThreadgroup: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setMaxTotalThreadsPerThreadgroup:"),
            maxTotalThreadsPerThreadgroup,
        );
    }

    /// `-threadGroupSizeIsMultipleOfThreadExecutionWidth` — note the upstream
    /// capital "Group" in the selector.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn threadGroupSizeIsMultipleOfThreadExecutionWidth(&self) -> bool {
        send_bool(
            self.as_raw(),
            sel!("threadGroupSizeIsMultipleOfThreadExecutionWidth"),
        )
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setThreadGroupSizeIsMultipleOfThreadExecutionWidth(
        &self,
        threadGroupSizeIsMultipleOfThreadExecutionWidth: bool,
    ) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setThreadGroupSizeIsMultipleOfThreadExecutionWidth:"),
            bool_arg(threadGroupSizeIsMultipleOfThreadExecutionWidth),
        );
    }

    /// `-requiredThreadsPerThreadgroup` (24-byte struct return ⇒ `_stret`
    /// on x86_64).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize {
        send_stret::<MTLSize>(self.as_raw(), sel!("requiredThreadsPerThreadgroup"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRequiredThreadsPerThreadgroup(&self, requiredThreadsPerThreadgroup: MTLSize) {
        send1::<MTLSize, ()>(
            self.as_raw(),
            sel!("setRequiredThreadsPerThreadgroup:"),
            requiredThreadsPerThreadgroup,
        );
    }

    /// `-shaderValidation` / `-setShaderValidation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn shaderValidation(&self) -> MTLShaderValidation {
        send::<MTLShaderValidation>(self.as_raw(), sel!("shaderValidation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setShaderValidation(&self, shaderValidation: MTLShaderValidation) {
        send1::<MTLShaderValidation, ()>(self.as_raw(), sel!("setShaderValidation:"), shaderValidation);
    }

    /// `-supportIndirectCommandBuffers` / `-setSupportIndirectCommandBuffers:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportIndirectCommandBuffers(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportIndirectCommandBuffers"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSupportIndirectCommandBuffers(&self, supportIndirectCommandBuffers: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setSupportIndirectCommandBuffers:"),
            bool_arg(supportIndirectCommandBuffers),
        );
    }
}

objc_class!(
    /// `MTL::ComputePipelineState` (`Metal/MTLComputePipeline.hpp`).
    pub struct MTLComputePipelineState : "MTLComputePipelineState";
);

impl MTLComputePipelineState {
    /// `-threadExecutionWidth`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn threadExecutionWidth(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("threadExecutionWidth"))
    }

    /// `-maxTotalThreadsPerThreadgroup`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxTotalThreadsPerThreadgroup"))
    }

    /// `-staticThreadgroupMemoryLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn staticThreadgroupMemoryLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("staticThreadgroupMemoryLength"))
    }

    /// `-requiredThreadsPerThreadgroup` (24-byte struct return ⇒ `_stret`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize {
        send_stret::<MTLSize>(self.as_raw(), sel!("requiredThreadsPerThreadgroup"))
    }

    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-gpuResourceID`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn gpuResourceID(&self) -> MTLResourceID {
        send::<MTLResourceID>(self.as_raw(), sel!("gpuResourceID"))
    }

    /// `-shaderValidation`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn shaderValidation(&self) -> MTLShaderValidation {
        send::<MTLShaderValidation>(self.as_raw(), sel!("shaderValidation"))
    }

    /// `-supportIndirectCommandBuffers`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportIndirectCommandBuffers(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportIndirectCommandBuffers"))
    }
}

// ---------------------------------------------------------------------------
// MTL::DepthStencilDescriptor / MTL::StencilDescriptor / MTL::DepthStencilState
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::StencilDescriptor` (`Metal/MTLDepthStencil.hpp`).
    pub struct MTLStencilDescriptor : "MTLStencilDescriptor";
);

impl MTLStencilDescriptor {
    /// `-stencilCompareFunction` / `-setStencilCompareFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stencilCompareFunction(&self) -> MTLCompareFunction {
        send::<MTLCompareFunction>(self.as_raw(), sel!("stencilCompareFunction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilCompareFunction(&self, stencilCompareFunction: MTLCompareFunction) {
        send1::<MTLCompareFunction, ()>(
            self.as_raw(),
            sel!("setStencilCompareFunction:"),
            stencilCompareFunction,
        );
    }

    /// `-stencilFailureOperation` / `-setStencilFailureOperation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stencilFailureOperation(&self) -> MTLStencilOperation {
        send::<MTLStencilOperation>(self.as_raw(), sel!("stencilFailureOperation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStencilFailureOperation(&self, stencilFailureOperation: MTLStencilOperation) {
        send1::<MTLStencilOperation, ()>(
            self.as_raw(),
            sel!("setStencilFailureOperation:"),
            stencilFailureOperation,
        );
    }

    /// `-depthFailureOperation` / `-setDepthFailureOperation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthFailureOperation(&self) -> MTLStencilOperation {
        send::<MTLStencilOperation>(self.as_raw(), sel!("depthFailureOperation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthFailureOperation(&self, depthFailureOperation: MTLStencilOperation) {
        send1::<MTLStencilOperation, ()>(
            self.as_raw(),
            sel!("setDepthFailureOperation:"),
            depthFailureOperation,
        );
    }

    /// `-depthStencilPassOperation` / `-setDepthStencilPassOperation:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthStencilPassOperation(&self) -> MTLStencilOperation {
        send::<MTLStencilOperation>(self.as_raw(), sel!("depthStencilPassOperation"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthStencilPassOperation(&self, depthStencilPassOperation: MTLStencilOperation) {
        send1::<MTLStencilOperation, ()>(
            self.as_raw(),
            sel!("setDepthStencilPassOperation:"),
            depthStencilPassOperation,
        );
    }

    /// `-readMask` / `-setReadMask:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn readMask(&self) -> u32 {
        send::<u32>(self.as_raw(), sel!("readMask"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setReadMask(&self, readMask: u32) {
        send1::<u32, ()>(self.as_raw(), sel!("setReadMask:"), readMask);
    }

    /// `-writeMask` / `-setWriteMask:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn writeMask(&self) -> u32 {
        send::<u32>(self.as_raw(), sel!("writeMask"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setWriteMask(&self, writeMask: u32) {
        send1::<u32, ()>(self.as_raw(), sel!("setWriteMask:"), writeMask);
    }
}

objc_class!(
    /// `MTL::DepthStencilDescriptor` (`Metal/MTLDepthStencil.hpp`).
    pub struct MTLDepthStencilDescriptor : "MTLDepthStencilDescriptor";
);

impl MTLDepthStencilDescriptor {
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

    /// `-depthCompareFunction` / `-setDepthCompareFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depthCompareFunction(&self) -> MTLCompareFunction {
        send::<MTLCompareFunction>(self.as_raw(), sel!("depthCompareFunction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthCompareFunction(&self, depthCompareFunction: MTLCompareFunction) {
        send1::<MTLCompareFunction, ()>(
            self.as_raw(),
            sel!("setDepthCompareFunction:"),
            depthCompareFunction,
        );
    }

    /// `-isDepthWriteEnabled` / `-setDepthWriteEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isDepthWriteEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isDepthWriteEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepthWriteEnabled(&self, depthWriteEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setDepthWriteEnabled:"),
            bool_arg(depthWriteEnabled),
        );
    }

    /// `-frontFaceStencil` / `-setFrontFaceStencil:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn frontFaceStencil(&self) -> Ref<MTLStencilDescriptor> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("frontFaceStencil")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFrontFaceStencil(&self, frontFaceStencil: Ref<MTLStencilDescriptor>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setFrontFaceStencil:"), frontFaceStencil.as_raw());
    }

    /// `-backFaceStencil` / `-setBackFaceStencil:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn backFaceStencil(&self) -> Ref<MTLStencilDescriptor> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("backFaceStencil")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBackFaceStencil(&self, backFaceStencil: Ref<MTLStencilDescriptor>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setBackFaceStencil:"), backFaceStencil.as_raw());
    }
}

objc_class!(
    /// `MTL::DepthStencilState` (`Metal/MTLDepthStencil.hpp`).
    pub struct MTLDepthStencilState : "MTLDepthStencilState";
);

impl MTLDepthStencilState {
    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }

    /// `-gpuResourceID`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn gpuResourceID(&self) -> MTLResourceID {
        send::<MTLResourceID>(self.as_raw(), sel!("gpuResourceID"))
    }
}

// ---------------------------------------------------------------------------
// MTL::VertexDescriptor family
// ---------------------------------------------------------------------------

/// `MTL::BufferLayoutStrideDynamic` — `NS::UIntegerMax`.
pub const MTLBufferLayoutStrideDynamic: NSUInteger = NSUInteger::MAX;

objc_class!(
    /// `MTL::VertexBufferLayoutDescriptor`.
    pub struct MTLVertexBufferLayoutDescriptor : "MTLVertexBufferLayoutDescriptor";
);

impl MTLVertexBufferLayoutDescriptor {
    /// `-stride` / `-setStride:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stride(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("stride"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStride(&self, stride: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setStride:"), stride);
    }

    /// `-stepFunction` / `-setStepFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stepFunction(&self) -> MTLVertexStepFunction {
        send::<MTLVertexStepFunction>(self.as_raw(), sel!("stepFunction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStepFunction(&self, stepFunction: MTLVertexStepFunction) {
        send1::<MTLVertexStepFunction, ()>(self.as_raw(), sel!("setStepFunction:"), stepFunction);
    }

    /// `-stepRate` / `-setStepRate:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stepRate(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("stepRate"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStepRate(&self, stepRate: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setStepRate:"), stepRate);
    }
}

objc_class!(
    /// `MTL::VertexBufferLayoutDescriptorArray`.
    pub struct MTLVertexBufferLayoutDescriptorArray : "MTLVertexBufferLayoutDescriptorArray";
);

impl MTLVertexBufferLayoutDescriptorArray {
    /// `-objectAtIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectAtIndexedSubscript(&self, index: NSUInteger) -> Ref<MTLVertexBufferLayoutDescriptor> {
        Ref::from_raw(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("objectAtIndexedSubscript:"),
            index,
        ))
    }

    /// `-setObject:atIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setObjectAtIndexedSubscript(
        &self,
        bufferDesc: Ref<MTLVertexBufferLayoutDescriptor>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setObject:atIndexedSubscript:"),
            bufferDesc.as_raw(),
            index,
        );
    }
}

objc_class!(
    /// `MTL::VertexAttributeDescriptor`.
    pub struct MTLVertexAttributeDescriptor : "MTLVertexAttributeDescriptor";
);

impl MTLVertexAttributeDescriptor {
    /// `-format` / `-setFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn format(&self) -> MTLVertexFormat {
        send::<MTLVertexFormat>(self.as_raw(), sel!("format"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFormat(&self, format: MTLVertexFormat) {
        send1::<MTLVertexFormat, ()>(self.as_raw(), sel!("setFormat:"), format);
    }

    /// `-offset` / `-setOffset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn offset(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("offset"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setOffset(&self, offset: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setOffset:"), offset);
    }

    /// `-bufferIndex` / `-setBufferIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn bufferIndex(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("bufferIndex"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBufferIndex(&self, bufferIndex: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setBufferIndex:"), bufferIndex);
    }
}

objc_class!(
    /// `MTL::VertexAttributeDescriptorArray`.
    pub struct MTLVertexAttributeDescriptorArray : "MTLVertexAttributeDescriptorArray";
);

impl MTLVertexAttributeDescriptorArray {
    /// `-objectAtIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectAtIndexedSubscript(&self, index: NSUInteger) -> Ref<MTLVertexAttributeDescriptor> {
        Ref::from_raw(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("objectAtIndexedSubscript:"),
            index,
        ))
    }

    /// `-setObject:atIndexedSubscript:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setObjectAtIndexedSubscript(
        &self,
        attributeDesc: Ref<MTLVertexAttributeDescriptor>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setObject:atIndexedSubscript:"),
            attributeDesc.as_raw(),
            index,
        );
    }
}

objc_class!(
    /// `MTL::VertexDescriptor` (`Metal/MTLVertexDescriptor.hpp`).
    pub struct MTLVertexDescriptor : "MTLVertexDescriptor";
);

impl MTLVertexDescriptor {
    /// `+[MTLVertexDescriptor vertexDescriptor]` (class factory, +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn vertexDescriptor() -> Strong<MTLVertexDescriptor> {
        Strong::from_retain(send::<Raw>(
            MTLVertexDescriptor::class().ptr(),
            sel!("vertexDescriptor"),
        ))
    }

    /// `-attributes`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn attributes(&self) -> Ref<MTLVertexAttributeDescriptorArray> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("attributes")))
    }

    /// `-layouts`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn layouts(&self) -> Ref<MTLVertexBufferLayoutDescriptorArray> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("layouts")))
    }

    /// `-reset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn reset(&self) {
        send::<()>(self.as_raw(), sel!("reset"));
    }
}

// ---------------------------------------------------------------------------
// MTL::BinaryArchive (minimal — add* selectors intentionally omitted)
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::BinaryArchiveDescriptor` (`Metal/MTLBinaryArchive.hpp`).
    pub struct MTLBinaryArchiveDescriptor : "MTLBinaryArchiveDescriptor";
);

impl MTLBinaryArchiveDescriptor {
    /// `-url` / `-setUrl:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn url(&self) -> Ref<NSURL> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("url")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setUrl(&self, url: Ref<NSURL>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setUrl:"), url.as_raw());
    }
}

objc_class!(
    /// `MTL::BinaryArchive` (`Metal/MTLBinaryArchive.hpp`) — only the verified
    /// core surface is mapped (`add*` methods are intentionally omitted).
    pub struct MTLBinaryArchive : "MTLBinaryArchive";
);

impl MTLBinaryArchive {
    /// `-serializeToURL:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn serializeToURL(&self, url: Ref<NSURL>, error: &mut Strong<NSError>) -> bool {
        let mut err: Raw = std::ptr::null_mut();
        let ok = send2::<Raw, *mut Raw, crate::objc_abi::ObjBOOL>(
            self.as_raw(),
            sel!("serializeToURL:error:"),
            url.as_raw(),
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

    /// `-device`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn device(&self) -> Ref<MTLDevice> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("device")))
    }
}
