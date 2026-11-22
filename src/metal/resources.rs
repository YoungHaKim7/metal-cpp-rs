//! GPU resources — `MTL::Resource`, `MTL::Buffer`, `MTL::Texture`,
//! `MTL::SamplerState`, `MTL::Heap` and their descriptors
//! (`Metal/MTLResource.hpp`, `MTLBuffer.hpp`, `MTLTexture.hpp`,
//! `MTLSampler.hpp`, `MTLHeap.hpp`).

use crate::foundation::{NSRange, NSString};
use crate::metal::device::MTLDevice;
use crate::metal::enums::{
    MTLBufferSparseTier, MTLCPUCacheMode, MTLHazardTrackingMode,
    MTLHeapType, MTLPixelFormat, MTLPurgeableState, MTLResourceOptions,
    MTLSamplerAddressMode, MTLSamplerBorderColor, MTLSamplerMinMagFilter, MTLSamplerMipFilter,
    MTLSamplerReductionMode, MTLSparsePageSize, MTLStorageMode, MTLTextureCompressionType,
    MTLTextureSparseTier, MTLTextureType, MTLTextureUsage,
};
use crate::metal::types::{
    GPUAddress, MTLRegion, MTLResourceID, MTLTextureSwizzleChannels,
};
use crate::objc_abi::{
    bool_arg, send, send1, send2, send3, send4, send6, send_bool, send_f32, NSUInteger,
    Obj, Raw, Ref, Strong,
};
use crate::objc_abi::{objc_class, sel};
use std::ffi::c_void;

// ---------------------------------------------------------------------------
// MTL::Resource (base class of Buffer/Texture/Heap ...)
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::Resource` (`Metal/MTLResource.hpp`) — base of `Buffer`, `Texture`.
    pub struct MTLResource : "MTLResource";
);

impl MTLResource {
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

    /// `-allocatedSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn allocatedSize(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("allocatedSize"))
    }

    /// `-cpuCacheMode`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode {
        send::<MTLCPUCacheMode>(self.as_raw(), sel!("cpuCacheMode"))
    }

    /// `-storageMode`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn storageMode(&self) -> MTLStorageMode {
        send::<MTLStorageMode>(self.as_raw(), sel!("storageMode"))
    }

    /// `-hazardTrackingMode`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode {
        send::<MTLHazardTrackingMode>(self.as_raw(), sel!("hazardTrackingMode"))
    }

    /// `-resourceOptions`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resourceOptions(&self) -> MTLResourceOptions {
        send::<MTLResourceOptions>(self.as_raw(), sel!("resourceOptions"))
    }

    /// `-heap`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heap(&self) -> Ref<MTLHeap> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("heap")))
    }

    /// `-heapOffset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heapOffset(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("heapOffset"))
    }

    /// `-isAliasable`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isAliasable(&self) -> bool {
        send_bool(self.as_raw(), sel!("isAliasable"))
    }

    /// `-makeAliasable`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn makeAliasable(&self) {
        send::<()>(self.as_raw(), sel!("makeAliasable"));
    }

    /// `-setPurgeableState:` — returns the resulting state.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        send1::<MTLPurgeableState, MTLPurgeableState>(
            self.as_raw(),
            sel!("setPurgeableState:"),
            state,
        )
    }
}

// ---------------------------------------------------------------------------
// MTL::Buffer
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::Buffer` (`Metal/MTLBuffer.hpp`).
    pub struct MTLBuffer : "MTLBuffer";
);

impl MTLBuffer {
    /// Non-owning upcast to the `MTLResource` base.
    pub unsafe fn as_resource(&self) -> Ref<MTLResource> {
        Ref::from_raw(self.as_raw())
    }

    /// `-length`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn length(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("length"))
    }

    /// `-contents` — a pointer to the CPU-visible storage (valid until the
    /// buffer is released; for `StorageModeManaged` call `didModifyRange`
    /// after writes).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn contents(&self) -> *mut c_void {
        send::<*mut c_void>(self.as_raw(), sel!("contents"))
    }

    /// `-gpuAddress`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn gpuAddress(&self) -> GPUAddress {
        send::<GPUAddress>(self.as_raw(), sel!("gpuAddress"))
    }

    /// `-didModifyRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn didModifyRange(&self, range: NSRange) {
        send1::<NSRange, ()>(self.as_raw(), sel!("didModifyRange:"), range);
    }

    /// `-newTextureWithDescriptor:offset:bytesPerRow:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTexture(
        &self,
        descriptor: Ref<MTLTextureDescriptor>,
        offset: NSUInteger,
        bytesPerRow: NSUInteger,
    ) -> Strong<MTLTexture> {
        Strong::from_transfer(send3::<Raw, NSUInteger, NSUInteger, Raw>(
            self.as_raw(),
            sel!("newTextureWithDescriptor:offset:bytesPerRow:"),
            descriptor.as_raw(),
            offset,
            bytesPerRow,
        ))
    }

    /// `-addDebugMarker:range:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn addDebugMarker(&self, marker: Ref<NSString>, range: NSRange) {
        send2::<Raw, NSRange, ()>(self.as_raw(), sel!("addDebugMarker:range:"), marker.as_raw(), range);
    }

    /// `-removeAllDebugMarkers`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn removeAllDebugMarkers(&self) {
        send::<()>(self.as_raw(), sel!("removeAllDebugMarkers"));
    }

    /// `-remoteStorageBuffer`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn remoteStorageBuffer(&self) -> Ref<MTLBuffer> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("remoteStorageBuffer")))
    }

    /// `-sparseBufferTier`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sparseBufferTier(&self) -> MTLBufferSparseTier {
        send::<MTLBufferSparseTier>(self.as_raw(), sel!("sparseBufferTier"))
    }
}

// ---------------------------------------------------------------------------
// MTL::Texture
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::Texture` (`Metal/MTLTexture.hpp`).
    pub struct MTLTexture : "MTLTexture";
);

impl MTLTexture {
    /// Non-owning upcast to the `MTLResource` base.
    pub unsafe fn as_resource(&self) -> Ref<MTLResource> {
        Ref::from_raw(self.as_raw())
    }

    /// `-width`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn width(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("width"))
    }

    /// `-height`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn height(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("height"))
    }

    /// `-depth`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depth(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("depth"))
    }

    /// `-arrayLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn arrayLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("arrayLength"))
    }

    /// `-sampleCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sampleCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("sampleCount"))
    }

    /// `-mipmapLevelCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mipmapLevelCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("mipmapLevelCount"))
    }

    /// `-pixelFormat`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn pixelFormat(&self) -> MTLPixelFormat {
        send::<MTLPixelFormat>(self.as_raw(), sel!("pixelFormat"))
    }

    /// `-textureType`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn textureType(&self) -> MTLTextureType {
        send::<MTLTextureType>(self.as_raw(), sel!("textureType"))
    }

    /// `-usage`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn usage(&self) -> MTLTextureUsage {
        send::<MTLTextureUsage>(self.as_raw(), sel!("usage"))
    }

    /// `-isFramebufferOnly`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isFramebufferOnly(&self) -> bool {
        send_bool(self.as_raw(), sel!("isFramebufferOnly"))
    }

    /// `-isShareable`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isShareable(&self) -> bool {
        send_bool(self.as_raw(), sel!("isShareable"))
    }

    /// `-allowGPUOptimizedContents`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn allowGPUOptimizedContents(&self) -> bool {
        send_bool(self.as_raw(), sel!("allowGPUOptimizedContents"))
    }

    /// `-gpuResourceID` — returns the 8-byte `MTL::ResourceID`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn gpuResourceID(&self) -> MTLResourceID {
        send::<MTLResourceID>(self.as_raw(), sel!("gpuResourceID"))
    }

    /// `-minLOD` (`float` return ⇒ `objc_msgSend_fpret` on x86_64).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn minLOD(&self) -> f32 {
        send_f32(self.as_raw(), sel!("minLOD"))
    }

    /// `-swizzle` (4-byte struct — registers, no `_stret`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn swizzle(&self) -> MTLTextureSwizzleChannels {
        send::<MTLTextureSwizzleChannels>(self.as_raw(), sel!("swizzle"))
    }

    /// `-getBytes:bytesPerRow:fromRegion:mipmapLevel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn getBytes(
        &self,
        pixelBytes: *mut c_void,
        bytesPerRow: NSUInteger,
        region: MTLRegion,
        level: NSUInteger,
    ) {
        send4::<*mut c_void, NSUInteger, MTLRegion, NSUInteger, ()>(
            self.as_raw(),
            sel!("getBytes:bytesPerRow:fromRegion:mipmapLevel:"),
            pixelBytes,
            bytesPerRow,
            region,
            level,
        );
    }

    /// `-getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn getBytesSlice(
        &self,
        pixelBytes: *mut c_void,
        bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger,
        region: MTLRegion,
        level: NSUInteger,
        slice: NSUInteger,
    ) {
        send6::<*mut c_void, NSUInteger, NSUInteger, MTLRegion, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:"),
            pixelBytes,
            bytesPerRow,
            bytesPerImage,
            region,
            level,
            slice,
        );
    }

    /// `-replaceRegion:mipmapLevel:withBytes:bytesPerRow:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn replaceRegion(
        &self,
        region: MTLRegion,
        level: NSUInteger,
        pixelBytes: *const c_void,
        bytesPerRow: NSUInteger,
    ) {
        send4::<MTLRegion, NSUInteger, *const c_void, NSUInteger, ()>(
            self.as_raw(),
            sel!("replaceRegion:mipmapLevel:withBytes:bytesPerRow:"),
            region,
            level,
            pixelBytes,
            bytesPerRow,
        );
    }

    /// `-replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn replaceRegionSlice(
        &self,
        region: MTLRegion,
        level: NSUInteger,
        slice: NSUInteger,
        pixelBytes: *const c_void,
        bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger,
    ) {
        send6::<MTLRegion, NSUInteger, NSUInteger, *const c_void, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:"),
            region,
            level,
            slice,
            pixelBytes,
            bytesPerRow,
            bytesPerImage,
        );
    }

    /// `-newTextureViewWithPixelFormat:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTextureView(&self, pixelFormat: MTLPixelFormat) -> Strong<MTLTexture> {
        Strong::from_transfer(send1::<MTLPixelFormat, Raw>(
            self.as_raw(),
            sel!("newTextureViewWithPixelFormat:"),
            pixelFormat,
        ))
    }

    /// `-newTextureViewWithPixelFormat:textureType:levels:slices:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTextureViewRange(
        &self,
        pixelFormat: MTLPixelFormat,
        textureType: MTLTextureType,
        levelRange: NSRange,
        sliceRange: NSRange,
    ) -> Strong<MTLTexture> {
        Strong::from_transfer(send4::<MTLPixelFormat, MTLTextureType, NSRange, NSRange, Raw>(
            self.as_raw(),
            sel!("newTextureViewWithPixelFormat:textureType:levels:slices:"),
            pixelFormat,
            textureType,
            levelRange,
            sliceRange,
        ))
    }

    /// `-parentTexture`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn parentTexture(&self) -> Ref<MTLTexture> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("parentTexture")))
    }

    /// `-rootResource`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn rootResource(&self) -> Ref<MTLResource> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("rootResource")))
    }

    /// `-buffer` (when the texture is a buffer view)
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn buffer(&self) -> Ref<MTLBuffer> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("buffer")))
    }

    /// `-bufferOffset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn bufferOffset(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("bufferOffset"))
    }

    /// `-bufferBytesPerRow`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn bufferBytesPerRow(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("bufferBytesPerRow"))
    }

    /// `-iosurface` (raw `IOSurfaceRef`, opaque in core scope).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn iosurface(&self) -> Raw {
        send::<Raw>(self.as_raw(), sel!("iosurface"))
    }

    /// `-iosurfacePlane`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn iosurfacePlane(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("iosurfacePlane"))
    }

    /// `-sparseTextureTier`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sparseTextureTier(&self) -> MTLTextureSparseTier {
        send::<MTLTextureSparseTier>(self.as_raw(), sel!("sparseTextureTier"))
    }
}

// ---------------------------------------------------------------------------
// MTL::TextureDescriptor
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::TextureDescriptor` (`Metal/MTLTexture.hpp`).
    pub struct MTLTextureDescriptor : "MTLTextureDescriptor";
);

impl MTLTextureDescriptor {
    /// `+[MTLTextureDescriptor texture2DDescriptorWithPixelFormat:width:height:mipmapped:]`
    /// (class method, autoreleased +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn texture2DDescriptor(
        pixelFormat: MTLPixelFormat,
        width: NSUInteger,
        height: NSUInteger,
        mipmapped: bool,
    ) -> Strong<MTLTextureDescriptor> {
        Strong::from_retain(send4::<MTLPixelFormat, NSUInteger, NSUInteger, crate::objc_abi::ObjBOOL, Raw>(
            Self::class_ptr().ptr(),
            sel!("texture2DDescriptorWithPixelFormat:width:height:mipmapped:"),
            pixelFormat,
            width,
            height,
            bool_arg(mipmapped),
        ))
    }

    /// `+[MTLTextureDescriptor textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn textureBufferDescriptor(
        pixelFormat: MTLPixelFormat,
        width: NSUInteger,
        resourceOptions: MTLResourceOptions,
        usage: MTLTextureUsage,
    ) -> Strong<MTLTextureDescriptor> {
        Strong::from_retain(send4::<MTLPixelFormat, NSUInteger, MTLResourceOptions, MTLTextureUsage, Raw>(
            Self::class_ptr().ptr(),
            sel!("textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:"),
            pixelFormat,
            width,
            resourceOptions,
            usage,
        ))
    }

    /// `+[MTLTextureDescriptor textureCubeDescriptorWithPixelFormat:size:mipmapped:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn textureCubeDescriptor(
        pixelFormat: MTLPixelFormat,
        size: NSUInteger,
        mipmapped: bool,
    ) -> Strong<MTLTextureDescriptor> {
        Strong::from_retain(send3::<MTLPixelFormat, NSUInteger, crate::objc_abi::ObjBOOL, Raw>(
            Self::class_ptr().ptr(),
            sel!("textureCubeDescriptorWithPixelFormat:size:mipmapped:"),
            pixelFormat,
            size,
            bool_arg(mipmapped),
        ))
    }

    /// `-width` / `-setWidth:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn width(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("width"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setWidth(&self, width: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setWidth:"), width);
    }

    /// `-height` / `-setHeight:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn height(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("height"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setHeight(&self, height: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setHeight:"), height);
    }

    /// `-depth` / `-setDepth:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn depth(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("depth"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDepth(&self, depth: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setDepth:"), depth);
    }

    /// `-arrayLength` / `-setArrayLength:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn arrayLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("arrayLength"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setArrayLength(&self, arrayLength: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setArrayLength:"), arrayLength);
    }

    /// `-mipmapLevelCount` / `-setMipmapLevelCount:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mipmapLevelCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("mipmapLevelCount"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMipmapLevelCount(&self, mipmapLevelCount: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setMipmapLevelCount:"), mipmapLevelCount);
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

    /// `-pixelFormat` / `-setPixelFormat:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn pixelFormat(&self) -> MTLPixelFormat {
        send::<MTLPixelFormat>(self.as_raw(), sel!("pixelFormat"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPixelFormat(&self, pixelFormat: MTLPixelFormat) {
        send1::<MTLPixelFormat, ()>(self.as_raw(), sel!("setPixelFormat:"), pixelFormat);
    }

    /// `-textureType` / `-setTextureType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn textureType(&self) -> MTLTextureType {
        send::<MTLTextureType>(self.as_raw(), sel!("textureType"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTextureType(&self, textureType: MTLTextureType) {
        send1::<MTLTextureType, ()>(self.as_raw(), sel!("setTextureType:"), textureType);
    }

    /// `-storageMode` / `-setStorageMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn storageMode(&self) -> MTLStorageMode {
        send::<MTLStorageMode>(self.as_raw(), sel!("storageMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStorageMode(&self, storageMode: MTLStorageMode) {
        send1::<MTLStorageMode, ()>(self.as_raw(), sel!("setStorageMode:"), storageMode);
    }

    /// `-cpuCacheMode` / `-setCpuCacheMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode {
        send::<MTLCPUCacheMode>(self.as_raw(), sel!("cpuCacheMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCpuCacheMode(&self, cpuCacheMode: MTLCPUCacheMode) {
        send1::<MTLCPUCacheMode, ()>(self.as_raw(), sel!("setCpuCacheMode:"), cpuCacheMode);
    }

    /// `-hazardTrackingMode` / `-setHazardTrackingMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode {
        send::<MTLHazardTrackingMode>(self.as_raw(), sel!("hazardTrackingMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setHazardTrackingMode(&self, hazardTrackingMode: MTLHazardTrackingMode) {
        send1::<MTLHazardTrackingMode, ()>(
            self.as_raw(),
            sel!("setHazardTrackingMode:"),
            hazardTrackingMode,
        );
    }

    /// `-resourceOptions` / `-setResourceOptions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resourceOptions(&self) -> MTLResourceOptions {
        send::<MTLResourceOptions>(self.as_raw(), sel!("resourceOptions"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResourceOptions(&self, resourceOptions: MTLResourceOptions) {
        send1::<MTLResourceOptions, ()>(self.as_raw(), sel!("setResourceOptions:"), resourceOptions);
    }

    /// `-usage` / `-setUsage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn usage(&self) -> MTLTextureUsage {
        send::<MTLTextureUsage>(self.as_raw(), sel!("usage"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setUsage(&self, usage: MTLTextureUsage) {
        send1::<MTLTextureUsage, ()>(self.as_raw(), sel!("setUsage:"), usage);
    }

    /// `-swizzle` / `-setSwizzle:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn swizzle(&self) -> MTLTextureSwizzleChannels {
        send::<MTLTextureSwizzleChannels>(self.as_raw(), sel!("swizzle"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSwizzle(&self, swizzle: MTLTextureSwizzleChannels) {
        send1::<MTLTextureSwizzleChannels, ()>(self.as_raw(), sel!("setSwizzle:"), swizzle);
    }

    /// `-compressionType` / `-setCompressionType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn compressionType(&self) -> MTLTextureCompressionType {
        send::<MTLTextureCompressionType>(self.as_raw(), sel!("compressionType"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCompressionType(&self, compressionType: MTLTextureCompressionType) {
        send1::<MTLTextureCompressionType, ()>(
            self.as_raw(),
            sel!("setCompressionType:"),
            compressionType,
        );
    }

    /// `-allowGPUOptimizedContents` / `-setAllowGPUOptimizedContents:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn allowGPUOptimizedContents(&self) -> bool {
        send_bool(self.as_raw(), sel!("allowGPUOptimizedContents"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAllowGPUOptimizedContents(&self, allowGPUOptimizedContents: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setAllowGPUOptimizedContents:"),
            bool_arg(allowGPUOptimizedContents),
        );
    }

    /// `-placementSparsePageSize` / `-setPlacementSparsePageSize:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn placementSparsePageSize(&self) -> MTLSparsePageSize {
        send::<MTLSparsePageSize>(self.as_raw(), sel!("placementSparsePageSize"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPlacementSparsePageSize(&self, placementSparsePageSize: MTLSparsePageSize) {
        send1::<MTLSparsePageSize, ()>(
            self.as_raw(),
            sel!("setPlacementSparsePageSize:"),
            placementSparsePageSize,
        );
    }
}

// ---------------------------------------------------------------------------
// MTLSampler.hpp
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::SamplerDescriptor` (`Metal/MTLSampler.hpp`).
    pub struct MTLSamplerDescriptor : "MTLSamplerDescriptor";
);

impl MTLSamplerDescriptor {
    /// `-minFilter` / `-setMinFilter:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn minFilter(&self) -> MTLSamplerMinMagFilter {
        send::<MTLSamplerMinMagFilter>(self.as_raw(), sel!("minFilter"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMinFilter(&self, minFilter: MTLSamplerMinMagFilter) {
        send1::<MTLSamplerMinMagFilter, ()>(self.as_raw(), sel!("setMinFilter:"), minFilter);
    }

    /// `-magFilter` / `-setMagFilter:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn magFilter(&self) -> MTLSamplerMinMagFilter {
        send::<MTLSamplerMinMagFilter>(self.as_raw(), sel!("magFilter"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMagFilter(&self, magFilter: MTLSamplerMinMagFilter) {
        send1::<MTLSamplerMinMagFilter, ()>(self.as_raw(), sel!("setMagFilter:"), magFilter);
    }

    /// `-mipFilter` / `-setMipFilter:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mipFilter(&self) -> MTLSamplerMipFilter {
        send::<MTLSamplerMipFilter>(self.as_raw(), sel!("mipFilter"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMipFilter(&self, mipFilter: MTLSamplerMipFilter) {
        send1::<MTLSamplerMipFilter, ()>(self.as_raw(), sel!("setMipFilter:"), mipFilter);
    }

    /// `-sAddressMode` / `-setSAddressMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn sAddressMode(&self) -> MTLSamplerAddressMode {
        send::<MTLSamplerAddressMode>(self.as_raw(), sel!("sAddressMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSAddressMode(&self, sAddressMode: MTLSamplerAddressMode) {
        send1::<MTLSamplerAddressMode, ()>(self.as_raw(), sel!("setSAddressMode:"), sAddressMode);
    }

    /// `-tAddressMode` / `-setTAddressMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn tAddressMode(&self) -> MTLSamplerAddressMode {
        send::<MTLSamplerAddressMode>(self.as_raw(), sel!("tAddressMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTAddressMode(&self, tAddressMode: MTLSamplerAddressMode) {
        send1::<MTLSamplerAddressMode, ()>(self.as_raw(), sel!("setTAddressMode:"), tAddressMode);
    }

    /// `-rAddressMode` / `-setRAddressMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn rAddressMode(&self) -> MTLSamplerAddressMode {
        send::<MTLSamplerAddressMode>(self.as_raw(), sel!("rAddressMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRAddressMode(&self, rAddressMode: MTLSamplerAddressMode) {
        send1::<MTLSamplerAddressMode, ()>(self.as_raw(), sel!("setRAddressMode:"), rAddressMode);
    }

    /// `-borderColor` / `-setBorderColor:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn borderColor(&self) -> MTLSamplerBorderColor {
        send::<MTLSamplerBorderColor>(self.as_raw(), sel!("borderColor"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBorderColor(&self, borderColor: MTLSamplerBorderColor) {
        send1::<MTLSamplerBorderColor, ()>(self.as_raw(), sel!("setBorderColor:"), borderColor);
    }

    /// `-lodMinClamp` / `-setLodMinClamp:` (`float` arguments cross the ABI
    /// without `_fpret` — that variant is only for *returns*).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn lodMinClamp(&self) -> f32 {
        send_f32(self.as_raw(), sel!("lodMinClamp"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLodMinClamp(&self, lodMinClamp: f32) {
        send1::<f32, ()>(self.as_raw(), sel!("setLodMinClamp:"), lodMinClamp);
    }

    /// `-lodMaxClamp` / `-setLodMaxClamp:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn lodMaxClamp(&self) -> f32 {
        send_f32(self.as_raw(), sel!("lodMaxClamp"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLodMaxClamp(&self, lodMaxClamp: f32) {
        send1::<f32, ()>(self.as_raw(), sel!("setLodMaxClamp:"), lodMaxClamp);
    }

    /// `-lodBias` / `-setLodBias:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn lodBias(&self) -> f32 {
        send_f32(self.as_raw(), sel!("lodBias"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLodBias(&self, lodBias: f32) {
        send1::<f32, ()>(self.as_raw(), sel!("setLodBias:"), lodBias);
    }

    /// `-lodAverage` / `-setLodAverage:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn lodAverage(&self) -> bool {
        send_bool(self.as_raw(), sel!("lodAverage"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLodAverage(&self, lodAverage: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(self.as_raw(), sel!("setLodAverage:"), bool_arg(lodAverage));
    }

    /// `-normalizedCoordinates` / `-setNormalizedCoordinates:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn normalizedCoordinates(&self) -> bool {
        send_bool(self.as_raw(), sel!("normalizedCoordinates"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setNormalizedCoordinates(&self, normalizedCoordinates: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setNormalizedCoordinates:"),
            bool_arg(normalizedCoordinates),
        );
    }

    /// `-compareFunction` / `-setCompareFunction:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn compareFunction(&self) -> crate::metal::enums::MTLCompareFunction {
        send::<crate::metal::enums::MTLCompareFunction>(self.as_raw(), sel!("compareFunction"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCompareFunction(&self, compareFunction: crate::metal::enums::MTLCompareFunction) {
        send1::<crate::metal::enums::MTLCompareFunction, ()>(
            self.as_raw(),
            sel!("setCompareFunction:"),
            compareFunction,
        );
    }

    /// `-maxAnisotropy` / `-setMaxAnisotropy:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxAnisotropy(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("maxAnisotropy"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMaxAnisotropy(&self, maxAnisotropy: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setMaxAnisotropy:"), maxAnisotropy);
    }

    /// `-reductionMode` / `-setReductionMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn reductionMode(&self) -> MTLSamplerReductionMode {
        send::<MTLSamplerReductionMode>(self.as_raw(), sel!("reductionMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setReductionMode(&self, reductionMode: MTLSamplerReductionMode) {
        send1::<MTLSamplerReductionMode, ()>(self.as_raw(), sel!("setReductionMode:"), reductionMode);
    }

    /// `-supportArgumentBuffers` / `-setSupportArgumentBuffers:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn supportArgumentBuffers(&self) -> bool {
        send_bool(self.as_raw(), sel!("supportArgumentBuffers"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSupportArgumentBuffers(&self, supportArgumentBuffers: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setSupportArgumentBuffers:"),
            bool_arg(supportArgumentBuffers),
        );
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
    /// `MTL::SamplerState` (`Metal/MTLSampler.hpp`).
    pub struct MTLSamplerState : "MTLSamplerState";
);

impl MTLSamplerState {
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

    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }
}

// ---------------------------------------------------------------------------
// MTLHeap.hpp
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::HeapDescriptor` (`Metal/MTLHeap.hpp`).
    pub struct MTLHeapDescriptor : "MTLHeapDescriptor";
);

impl MTLHeapDescriptor {
    /// `-type` / `-setType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heapType(&self) -> MTLHeapType {
        send::<MTLHeapType>(self.as_raw(), sel!("type"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setType(&self, type_: MTLHeapType) {
        send1::<MTLHeapType, ()>(self.as_raw(), sel!("setType:"), type_);
    }

    /// `-size` / `-setSize:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn size(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("size"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSize(&self, size: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setSize:"), size);
    }

    /// `-storageMode` / `-setStorageMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn storageMode(&self) -> MTLStorageMode {
        send::<MTLStorageMode>(self.as_raw(), sel!("storageMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setStorageMode(&self, storageMode: MTLStorageMode) {
        send1::<MTLStorageMode, ()>(self.as_raw(), sel!("setStorageMode:"), storageMode);
    }

    /// `-cpuCacheMode` / `-setCpuCacheMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode {
        send::<MTLCPUCacheMode>(self.as_raw(), sel!("cpuCacheMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCpuCacheMode(&self, cpuCacheMode: MTLCPUCacheMode) {
        send1::<MTLCPUCacheMode, ()>(self.as_raw(), sel!("setCpuCacheMode:"), cpuCacheMode);
    }

    /// `-hazardTrackingMode` / `-setHazardTrackingMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode {
        send::<MTLHazardTrackingMode>(self.as_raw(), sel!("hazardTrackingMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setHazardTrackingMode(&self, hazardTrackingMode: MTLHazardTrackingMode) {
        send1::<MTLHazardTrackingMode, ()>(
            self.as_raw(),
            sel!("setHazardTrackingMode:"),
            hazardTrackingMode,
        );
    }

    /// `-resourceOptions` / `-setResourceOptions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resourceOptions(&self) -> MTLResourceOptions {
        send::<MTLResourceOptions>(self.as_raw(), sel!("resourceOptions"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setResourceOptions(&self, resourceOptions: MTLResourceOptions) {
        send1::<MTLResourceOptions, ()>(self.as_raw(), sel!("setResourceOptions:"), resourceOptions);
    }
}

objc_class!(
    /// `MTL::Heap` (`Metal/MTLHeap.hpp`).
    pub struct MTLHeap : "MTLHeap";
);

impl MTLHeap {
    /// `-type`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn heapType(&self) -> MTLHeapType {
        send::<MTLHeapType>(self.as_raw(), sel!("type"))
    }

    /// `-size`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn size(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("size"))
    }

    /// `-usedSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn usedSize(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("usedSize"))
    }

    /// `-currentAllocatedSize`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn currentAllocatedSize(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("currentAllocatedSize"))
    }

    /// `-maxAvailableSizeWithAlignment:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maxAvailableSize(&self, alignment: NSUInteger) -> NSUInteger {
        send1::<NSUInteger, NSUInteger>(
            self.as_raw(),
            sel!("maxAvailableSizeWithAlignment:"),
            alignment,
        )
    }

    /// `-newBufferWithLength:options:` (+1 → transferred).
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

    /// `-newTextureWithDescriptor:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newTexture(&self, descriptor: Ref<MTLTextureDescriptor>) -> Strong<MTLTexture> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newTextureWithDescriptor:"),
            descriptor.as_raw(),
        ))
    }

    /// `-setPurgeableState:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        send1::<MTLPurgeableState, MTLPurgeableState>(
            self.as_raw(),
            sel!("setPurgeableState:"),
            state,
        )
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
