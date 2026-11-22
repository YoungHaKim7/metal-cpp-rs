//! `MTL::Device` + `MTL::Architecture` (`Metal/MTLDevice.hpp`).

use std::ffi::c_void;

use crate::{
    blocks::Block2,
    foundation::{NSArray, NSError, NSString},
    metal::{
        command_queue::MTLCommandQueue,
        enums::{
            MTLArgumentBuffersTier, MTLCounterSamplingPoint, MTLDeviceLocation, MTLGPUFamily,
            MTLReadWriteTextureTier, MTLResourceOptions,
        },
        pipeline::{MTLComputePipelineState, MTLDepthStencilState, MTLRenderPipelineState},
        resources::{MTLBuffer, MTLHeap, MTLSamplerState, MTLTexture},
        shader::{MTLCompileOptions, MTLLibrary},
        sync::{MTLEvent, MTLFence, MTLSharedEvent},
        types::{MTLSize, MTLSizeAndAlign, Timestamp},
    },
    objc_abi::{
        NSUInteger, Obj, Raw, Ref, Strong, send, send_bool, send_stret, send1, send1_bool, send2,
        send3, send4, {objc_class, sel},
    },
};

objc_class!(
    /// `MTL::Architecture` — device architecture info.
    pub struct MTLArchitecture : "MTLArchitecture";
);

impl MTLArchitecture {
    /// `-name`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }
}

objc_class!(
    /// `MTL::Device` — the GPU handle (`Metal/MTLDevice.hpp`). All selectors
    /// are verbatim from metal-cpp's bridge (e.g. `newBufferWithLength:options:`).
    pub struct MTLDevice : "MTLDevice";
);

impl MTLDevice {
    // -- properties ----------------------------------------------------------

    /// `-name`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }

    /// `-architecture`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn architecture(&self) -> Strong<MTLArchitecture> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("architecture")))
    }

    /// `-registryID`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn registryID(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("registryID"))
    }

    /// `-hasUnifiedMemory`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn hasUnifiedMemory(&self) -> bool {
        send_bool(self.as_raw(), sel!("hasUnifiedMemory"))
    }

    /// `-isLowPower`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isLowPower(&self) -> bool {
        send_bool(self.as_raw(), sel!("isLowPower"))
    }

    /// `-isHeadless`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isHeadless(&self) -> bool {
        send_bool(self.as_raw(), sel!("isHeadless"))
    }

    /// `-isRemovable`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isRemovable(&self) -> bool {
        send_bool(self.as_raw(), sel!("isRemovable"))
    }

    /// `-location`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn location(&self) -> MTLDeviceLocation {
        send::<MTLDeviceLocation>(self.as_raw(), sel!("location"))
    }

    /// `-locationNumber`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn locationNumber(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("locationNumber"))
    }

    /// `-maxThreadsPerThreadgroup` — returns `MTL::Size` (24 bytes ⇒
    /// `objc_msgSend_stret` on x86_64).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxThreadsPerThreadgroup(&self) -> MTLSize {
        send_stret::<MTLSize>(self.as_raw(), sel!("maxThreadsPerThreadgroup"))
    }

    /// `-maxThreadgroupMemoryLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxThreadgroupMemoryLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxThreadgroupMemoryLength"))
    }

    /// `-maxBufferLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxBufferLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxBufferLength"))
    }

    /// `-recommendedMaxWorkingSetSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn recommendedMaxWorkingSetSize(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("recommendedMaxWorkingSetSize"))
    }

    /// `-currentAllocatedSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn currentAllocatedSize(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("currentAllocatedSize"))
    }

    /// `-maxTransferRate`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxTransferRate(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("maxTransferRate"))
    }

    /// `-queryTimestampFrequency`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn queryTimestampFrequency(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("queryTimestampFrequency"))
    }

    /// `-maxArgumentBufferSamplerCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxArgumentBufferSamplerCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxArgumentBufferSamplerCount"))
    }

    /// `-peerGroupID`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn peerGroupID(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("peerGroupID"))
    }

    /// `-peerIndex`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn peerIndex(&self) -> u32 {
        send::<u32>(self.as_raw(), sel!("peerIndex"))
    }

    /// `-peerCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn peerCount(&self) -> u32 {
        send::<u32>(self.as_raw(), sel!("peerCount"))
    }

    /// `-readWriteTextureSupport`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn readWriteTextureSupport(&self) -> MTLReadWriteTextureTier {
        send::<MTLReadWriteTextureTier>(self.as_raw(), sel!("readWriteTextureSupport"))
    }

    /// `-argumentBuffersSupport`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn argumentBuffersSupport(&self) -> MTLArgumentBuffersTier {
        send::<MTLArgumentBuffersTier>(self.as_raw(), sel!("argumentBuffersSupport"))
    }

    /// `-counterSets` — counter sets are not modeled in the core scope; the
    /// elements are returned as untyped `NSObject` refs.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn counterSets(&self) -> Ref<NSArray<crate::foundation::NSObject>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("counterSets")))
    }

    /// `-sampleTimestamps:gpuTimestamp:` (two out-parameters).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sampleTimestamps(
        &self,
        cpuTimestamp: &mut Timestamp,
        gpuTimestamp: &mut Timestamp,
    ) {
        send2::<*mut Timestamp, *mut Timestamp, ()>(
            self.as_raw(),
            sel!("sampleTimestamps:gpuTimestamp:"),
            cpuTimestamp,
            gpuTimestamp,
        );
    }

    // -- capability queries --------------------------------------------------

    /// `-supportsFamily:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsFamily(&self, gpuFamily: MTLGPUFamily) -> bool {
        send1_bool(self.as_raw(), sel!("supportsFamily:"), gpuFamily)
    }

    /// `-supportsTextureSampleCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsTextureSampleCount(&self, sampleCount: NSUInteger) -> bool {
        send1_bool(
            self.as_raw(),
            sel!("supportsTextureSampleCount:"),
            sampleCount,
        )
    }

    /// `-supportsRaytracing`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsRaytracing(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportsRaytracing"))
    }

    /// `-supports32BitMSAA`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supports32BitMSAA(&self) -> bool {
        send_bool(self.as_raw(), sel!("supports32BitMSAA"))
    }

    /// `-supports32BitFloatFiltering`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supports32BitFloatFiltering(&self) -> bool {
        send_bool(self.as_raw(), sel!("supports32BitFloatFiltering"))
    }

    /// `-supportsBCTextureCompression`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsBCTextureCompression(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportsBCTextureCompression"))
    }

    /// `-supportsPullModelInterpolation`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsPullModelInterpolation(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportsPullModelInterpolation"))
    }

    /// `-supportsShaderBarycentricCoordinates`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsShaderBarycentricCoordinates(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportsShaderBarycentricCoordinates"))
    }

    /// `-areRasterOrderGroupsSupported`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn areRasterOrderGroupsSupported(&self) -> bool {
        send_bool(self.as_raw(), sel!("areRasterOrderGroupsSupported"))
    }

    /// `-areProgrammableSamplePositionsSupported`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn areProgrammableSamplePositionsSupported(&self) -> bool {
        send_bool(
            self.as_raw(),
            sel!("areProgrammableSamplePositionsSupported"),
        )
    }

    /// `-areBarycentricCoordsSupported`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn areBarycentricCoordsSupported(&self) -> bool {
        send_bool(self.as_raw(), sel!("areBarycentricCoordsSupported"))
    }

    /// `-supportsFunctionPointers`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsFunctionPointers(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportsFunctionPointers"))
    }

    /// `-supportsCounterSampling:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportsCounterSampling(&self, samplingPoint: MTLCounterSamplingPoint) -> bool {
        send1_bool(
            self.as_raw(),
            sel!("supportsCounterSampling:"),
            samplingPoint,
        )
    }

    /// `-isDepth24Stencil8PixelFormatSupported`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isDepth24Stencil8PixelFormatSupported(&self) -> bool {
        send_bool(self.as_raw(), sel!("isDepth24Stencil8PixelFormatSupported"))
    }

    /// `-minimumLinearTextureAlignmentForPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn minimumLinearTextureAlignmentForPixelFormat(
        &self,
        format: crate::metal::enums::MTLPixelFormat,
    ) -> NSUInteger {
        send1::<crate::metal::enums::MTLPixelFormat, NSUInteger>(
            self.as_raw(),
            sel!("minimumLinearTextureAlignmentForPixelFormat:"),
            format,
        )
    }

    /// `-minimumTextureBufferAlignmentForPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn minimumTextureBufferAlignmentForPixelFormat(
        &self,
        format: crate::metal::enums::MTLPixelFormat,
    ) -> NSUInteger {
        send1::<crate::metal::enums::MTLPixelFormat, NSUInteger>(
            self.as_raw(),
            sel!("minimumTextureBufferAlignmentForPixelFormat:"),
            format,
        )
    }

    // -- heaps sizing --------------------------------------------------------

    /// `-heapBufferSizeAndAlignWithLength:options:` — 16-byte struct return
    /// (registers — no `_stret`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heapBufferSizeAndAlign(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> MTLSizeAndAlign {
        send2::<NSUInteger, MTLResourceOptions, MTLSizeAndAlign>(
            self.as_raw(),
            sel!("heapBufferSizeAndAlignWithLength:options:"),
            length,
            options,
        )
    }

    /// `-heapTextureSizeAndAlignWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heapTextureSizeAndAlign(
        &self,
        descriptor: Ref<crate::metal::resources::MTLTextureDescriptor>,
    ) -> MTLSizeAndAlign {
        send1::<Raw, MTLSizeAndAlign>(
            self.as_raw(),
            sel!("heapTextureSizeAndAlignWithDescriptor:"),
            descriptor.as_raw(),
        )
    }

    // -- object creation (new* → +1, Transferred into Strong) ----------------

    /// `-newCommandQueue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newCommandQueue(&self) -> Strong<MTLCommandQueue> {
        Strong::from_transfer(send::<Raw>(self.as_raw(), sel!("newCommandQueue")))
    }

    /// `-newCommandQueueWithMaxCommandBufferCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newCommandQueueWithMaxCommandBufferCount(
        &self,
        maxCommandBufferCount: NSUInteger,
    ) -> Strong<MTLCommandQueue> {
        Strong::from_transfer(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("newCommandQueueWithMaxCommandBufferCount:"),
            maxCommandBufferCount,
        ))
    }

    /// `-newBufferWithLength:options:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newBuffer(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> Strong<MTLBuffer> {
        Strong::from_transfer(send2::<NSUInteger, MTLResourceOptions, Raw>(
            self.as_raw(),
            sel!("newBufferWithLength:options:"),
            length,
            options,
        ))
    }

    /// `-newBufferWithBytes:length:options:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newBufferWithBytes(
        &self,
        pointer: *const c_void,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> Strong<MTLBuffer> {
        Strong::from_transfer(send3::<*const c_void, NSUInteger, MTLResourceOptions, Raw>(
            self.as_raw(),
            sel!("newBufferWithBytes:length:options:"),
            pointer,
            length,
            options,
        ))
    }

    /// `-newBufferWithBytesNoCopy:length:options:deallocator:` with a nil
    /// deallocator block.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime. The memory must stay valid and
    /// page-aligned for the lifetime of the buffer.
    pub unsafe fn newBufferWithBytesNoCopy(
        &self,
        pointer: *mut c_void,
        length: NSUInteger,
        options: MTLResourceOptions,
        alignment: NSUInteger,
    ) -> Strong<MTLBuffer> {
        Strong::from_transfer(send4::<
            *mut c_void,
            NSUInteger,
            MTLResourceOptions,
            NSUInteger,
            Raw,
        >(
            self.as_raw(),
            sel!("newBufferWithBytesNoCopy:length:options:alignment:deallocator:"),
            pointer,
            length,
            options,
            alignment,
        ))
    }

    /// `-newTextureWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTexture(
        &self,
        descriptor: Ref<crate::metal::resources::MTLTextureDescriptor>,
    ) -> Strong<MTLTexture> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newTextureWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-newTextureWithDescriptor:iosurface:plane:` (raw `IOSurfaceRef`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTextureWithIOSurface(
        &self,
        descriptor: Ref<crate::metal::resources::MTLTextureDescriptor>,
        iosurface: Raw,
        plane: NSUInteger,
    ) -> Strong<MTLTexture> {
        Strong::from_transfer(send3::<Raw, Raw, NSUInteger, Raw>(
            self.as_raw(),
            sel!("newTextureWithDescriptor:iosurface:plane:"),
            descriptor.as_raw(),
            iosurface,
            plane,
        ))
    }

    /// `-newSamplerStateWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newSamplerState(
        &self,
        descriptor: Ref<crate::metal::resources::MTLSamplerDescriptor>,
    ) -> Strong<MTLSamplerState> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newSamplerStateWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-newDefaultLibrary`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newDefaultLibrary(&self) -> Strong<MTLLibrary> {
        Strong::from_transfer(send::<Raw>(self.as_raw(), sel!("newDefaultLibrary")))
    }

    /// `-newLibraryWithFile:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newLibraryWithFile(
        &self,
        filepath: Ref<NSString>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLLibrary> {
        let mut err: Raw = std::ptr::null_mut();
        let lib = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newLibraryWithFile:error:"),
            filepath.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(lib)
    }

    /// `-newLibraryWithData:error:` — `data` is an opaque
    /// `dispatch_data_t` (libdispatch objects are not modeled in core scope).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newLibraryWithData(
        &self,
        data: Raw,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLLibrary> {
        let mut err: Raw = std::ptr::null_mut();
        let lib = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newLibraryWithData:error:"),
            data,
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(lib)
    }

    /// `-newLibraryWithSource:options:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newLibrary(
        &self,
        source: Ref<NSString>,
        options: Ref<MTLCompileOptions>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLLibrary> {
        let mut err: Raw = std::ptr::null_mut();
        let lib = send3::<Raw, Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newLibraryWithSource:options:error:"),
            source.as_raw(),
            options.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(lib)
    }

    /// `-newLibraryWithSource:options:completionHandler:` — the block receives
    /// `(MTLLibrary*, NSError*)`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newLibraryWithCompletionHandler<F: FnMut(Raw, Raw) + 'static>(
        &self,
        source: Ref<NSString>,
        options: Ref<MTLCompileOptions>,
        completionHandler: &Block2<F>,
    ) {
        send3::<Raw, Raw, Raw, ()>(
            self.as_raw(),
            sel!("newLibraryWithSource:options:completionHandler:"),
            source.as_raw(),
            options.as_raw(),
            completionHandler.as_raw(),
        );
    }

    /// `-newRenderPipelineStateWithDescriptor:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newRenderPipelineState(
        &self,
        descriptor: Ref<crate::metal::pipeline::MTLRenderPipelineDescriptor>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLRenderPipelineState> {
        let mut err: Raw = std::ptr::null_mut();
        let pso = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newRenderPipelineStateWithDescriptor:error:"),
            descriptor.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(pso)
    }

    /// `-newRenderPipelineStateWithDescriptor:options:reflection:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newRenderPipelineStateWithReflection(
        &self,
        descriptor: Ref<crate::metal::pipeline::MTLRenderPipelineDescriptor>,
        options: crate::metal::enums::MTLPipelineOption,
        reflection: *mut Raw,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLRenderPipelineState> {
        let mut err: Raw = std::ptr::null_mut();
        let pso = send4::<Raw, crate::metal::enums::MTLPipelineOption, *mut Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newRenderPipelineStateWithDescriptor:options:reflection:error:"),
            descriptor.as_raw(),
            options,
            reflection,
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(pso)
    }

    /// `-newComputePipelineStateWithFunction:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newComputePipelineState(
        &self,
        computeFunction: Ref<crate::metal::shader::MTLFunction>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLComputePipelineState> {
        let mut err: Raw = std::ptr::null_mut();
        let pso = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newComputePipelineStateWithFunction:error:"),
            computeFunction.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(pso)
    }

    /// `-newComputePipelineStateWithFunction:options:reflection:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newComputePipelineStateWithReflection(
        &self,
        computeFunction: Ref<crate::metal::shader::MTLFunction>,
        options: crate::metal::enums::MTLPipelineOption,
        reflection: *mut Raw,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLComputePipelineState> {
        let mut err: Raw = std::ptr::null_mut();
        let pso = send4::<Raw, crate::metal::enums::MTLPipelineOption, *mut Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newComputePipelineStateWithFunction:options:reflection:error:"),
            computeFunction.as_raw(),
            options,
            reflection,
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(pso)
    }

    /// `-newDepthStencilStateWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newDepthStencilState(
        &self,
        descriptor: Ref<crate::metal::pipeline::MTLDepthStencilDescriptor>,
    ) -> Strong<MTLDepthStencilState> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newDepthStencilStateWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-newHeapWithDescriptor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newHeap(
        &self,
        descriptor: Ref<crate::metal::resources::MTLHeapDescriptor>,
    ) -> Strong<MTLHeap> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newHeapWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-newFence`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newFence(&self) -> Strong<MTLFence> {
        Strong::from_transfer(send::<Raw>(self.as_raw(), sel!("newFence")))
    }

    /// `-newEvent`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newEvent(&self) -> Strong<MTLEvent> {
        Strong::from_transfer(send::<Raw>(self.as_raw(), sel!("newEvent")))
    }

    /// `-newSharedEvent`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newSharedEvent(&self) -> Strong<MTLSharedEvent> {
        Strong::from_transfer(send::<Raw>(self.as_raw(), sel!("newSharedEvent")))
    }

    /// `-newArgumentEncoderWithArguments:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newArgumentEncoder(
        &self,
        arguments: Ref<NSArray<crate::metal::argument::MTLArgumentDescriptor>>,
    ) -> Strong<crate::metal::argument::MTLArgumentEncoder> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newArgumentEncoderWithArguments:"),
            arguments.as_raw(),
        ))
    }

    /// `-newBinaryArchiveWithDescriptor:error:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newBinaryArchive(
        &self,
        descriptor: Ref<crate::metal::pipeline::MTLBinaryArchiveDescriptor>,
        error: &mut Strong<NSError>,
    ) -> Strong<crate::metal::pipeline::MTLBinaryArchive> {
        let mut err: Raw = std::ptr::null_mut();
        let archive = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newBinaryArchiveWithDescriptor:error:"),
            descriptor.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(archive)
    }
}
