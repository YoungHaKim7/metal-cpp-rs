//! Argument tables — `MTL::ArgumentDescriptor` and `MTL::ArgumentEncoder`
//! (`Metal/MTLArgument.hpp`, `Metal/MTLArgumentEncoder.hpp`).

use crate::foundation::NSString;
use crate::metal::device::MTLDevice;
use crate::metal::enums::{MTLBindingAccess, MTLDataType, MTLTextureType};
use crate::objc_abi::{NSUInteger, Obj, Raw, Ref, Strong, send, send1, send2, send3};
use crate::objc_abi::{objc_class, sel};
use std::ffi::c_void;

/// `MTL::ArgumentEncoder::AttributeStrideStatic` — `NS::UIntegerMax`.
pub const MTLAttributeStrideStatic: NSUInteger = NSUInteger::MAX;

objc_class!(
    /// `MTL::ArgumentDescriptor` (`Metal/MTLDevice.hpp` declares the class;
    /// selectors verified from the bridge).
    pub struct MTLArgumentDescriptor : "MTLArgumentDescriptor";
);

impl MTLArgumentDescriptor {
    /// `+[MTLArgumentDescriptor argumentDescriptor]` (class factory, +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn argumentDescriptor() -> Strong<MTLArgumentDescriptor> {
        Strong::from_retain(send::<Raw>(
            MTLArgumentDescriptor::class().ptr(),
            sel!("argumentDescriptor"),
        ))
    }

    /// `-access` / `-setAccess:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn access(&self) -> MTLBindingAccess {
        send::<MTLBindingAccess>(self.as_raw(), sel!("access"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAccess(&self, access: MTLBindingAccess) {
        send1::<MTLBindingAccess, ()>(self.as_raw(), sel!("setAccess:"), access);
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

    /// `-constantBlockAlignment` / `-setConstantBlockAlignment:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn constantBlockAlignment(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("constantBlockAlignment"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setConstantBlockAlignment(&self, constantBlockAlignment: NSUInteger) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setConstantBlockAlignment:"),
            constantBlockAlignment,
        );
    }

    /// `-dataType` / `-setDataType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dataType(&self) -> MTLDataType {
        send::<MTLDataType>(self.as_raw(), sel!("dataType"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setDataType(&self, dataType: MTLDataType) {
        send1::<MTLDataType, ()>(self.as_raw(), sel!("setDataType:"), dataType);
    }

    /// `-index` / `-setIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn index(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("index"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setIndex(&self, index: NSUInteger) {
        send1::<NSUInteger, ()>(self.as_raw(), sel!("setIndex:"), index);
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
}

objc_class!(
    /// `MTL::ArgumentEncoder` (`Metal/MTLArgumentEncoder.hpp`).
    pub struct MTLArgumentEncoder : "MTLArgumentEncoder";
);

impl MTLArgumentEncoder {
    /// `-encodedLength`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn encodedLength(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("encodedLength"))
    }

    /// `-alignment`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn alignment(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("alignment"))
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

    /// `-constantDataAtIndex:` — pointer into the argument buffer.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn constantData(&self, index: NSUInteger) -> *mut c_void {
        send1::<NSUInteger, *mut c_void>(self.as_raw(), sel!("constantDataAtIndex:"), index)
    }

    /// `-setArgumentBuffer:offset:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setArgumentBuffer(
        &self,
        argumentBuffer: Ref<crate::metal::resources::MTLBuffer>,
        offset: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setArgumentBuffer:offset:"),
            argumentBuffer.as_raw(),
            offset,
        );
    }

    /// `-setArgumentBuffer:startOffset:arrayElement:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setArgumentBufferArrayElement(
        &self,
        argumentBuffer: Ref<crate::metal::resources::MTLBuffer>,
        startOffset: NSUInteger,
        arrayElement: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setArgumentBuffer:startOffset:arrayElement:"),
            argumentBuffer.as_raw(),
            startOffset,
            arrayElement,
        );
    }

    /// `-setBuffer:offset:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setBuffer(
        &self,
        buffer: Ref<crate::metal::resources::MTLBuffer>,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        send3::<Raw, NSUInteger, NSUInteger, ()>(
            self.as_raw(),
            sel!("setBuffer:offset:atIndex:"),
            buffer.as_raw(),
            offset,
            index,
        );
    }

    /// `-setTexture:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setTexture(
        &self,
        texture: Ref<crate::metal::resources::MTLTexture>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setTexture:atIndex:"),
            texture.as_raw(),
            index,
        );
    }

    /// `-setSamplerState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSamplerState(
        &self,
        sampler: Ref<crate::metal::resources::MTLSamplerState>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setSamplerState:atIndex:"),
            sampler.as_raw(),
            index,
        );
    }

    /// `-setRenderPipelineState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setRenderPipelineState(
        &self,
        pipeline: Ref<crate::metal::pipeline::MTLRenderPipelineState>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setRenderPipelineState:atIndex:"),
            pipeline.as_raw(),
            index,
        );
    }

    /// `-setComputePipelineState:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setComputePipelineState(
        &self,
        pipeline: Ref<crate::metal::pipeline::MTLComputePipelineState>,
        index: NSUInteger,
    ) {
        send2::<Raw, NSUInteger, ()>(
            self.as_raw(),
            sel!("setComputePipelineState:atIndex:"),
            pipeline.as_raw(),
            index,
        );
    }

    /// `-newArgumentEncoderForBufferAtIndex:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newArgumentEncoder(&self, index: NSUInteger) -> Strong<MTLArgumentEncoder> {
        Strong::from_transfer(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("newArgumentEncoderForBufferAtIndex:"),
            index,
        ))
    }
}
