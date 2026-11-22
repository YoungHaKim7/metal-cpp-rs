//! Shaders — `MTL::Library`, `MTL::Function`, `MTL::VertexAttribute`,
//! `MTL::CompileOptions`, `MTL::FunctionDescriptor`,
//! `MTL::FunctionConstantValues` (`Metal/MTLLibrary.hpp`,
//! `MTLFunctionDescriptor.hpp`, `MTLFunctionConstantValues.hpp`).

use std::ffi::c_void;

use crate::{
    foundation::{NSArray, NSDictionary, NSError, NSObject, NSString},
    metal::{
        argument::MTLArgumentEncoder,
        device::MTLDevice,
        enums::{
            MTLCompileSymbolVisibility, MTLDataType, MTLFloatingPointConversionRoundingMode,
            MTLFunctionOptions, MTLFunctionType, MTLLanguageVersion, MTLLibraryOptimizationLevel,
            MTLLibraryType, MTLMathFloatingPointFunctions, MTLMathMode, MTLPatchType,
        },
        types::MTLSize,
    },
    objc_abi::{
        NSUInteger, Obj, Raw, Ref, Strong, bool_arg, send, send_bool, send_stret, send1, send2,
        send3, {objc_class, sel},
    },
};

// ---------------------------------------------------------------------------
// MTL::Library
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::Library` (`Metal/MTLLibrary.hpp`).
    pub struct MTLLibrary : "MTLLibrary";
);

impl MTLLibrary {
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

    /// `-functionNames`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn functionNames(&self) -> Ref<NSArray<NSString>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("functionNames")))
    }

    /// `-installName`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn installName(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("installName")))
    }

    /// `-type` (`MTLLibraryType`)
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn libraryType(&self) -> MTLLibraryType {
        send::<MTLLibraryType>(self.as_raw(), sel!("type"))
    }

    /// `-newFunctionWithName:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newFunction(&self, functionName: Ref<NSString>) -> Strong<MTLFunction> {
        Strong::from_transfer(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("newFunctionWithName:"),
            functionName.as_raw(),
        ))
    }

    /// `-newFunctionWithName:constantValues:error:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newFunctionWithConstantValues(
        &self,
        functionName: Ref<NSString>,
        constantValues: Ref<MTLFunctionConstantValues>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLFunction> {
        let mut err: Raw = std::ptr::null_mut();
        let function = send3::<Raw, Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newFunctionWithName:constantValues:error:"),
            functionName.as_raw(),
            constantValues.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(function)
    }

    /// `-newFunctionWithDescriptor:error:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newFunctionWithDescriptor(
        &self,
        descriptor: Ref<MTLFunctionDescriptor>,
        error: &mut Strong<NSError>,
    ) -> Strong<MTLFunction> {
        let mut err: Raw = std::ptr::null_mut();
        let function = send2::<Raw, *mut Raw, Raw>(
            self.as_raw(),
            sel!("newFunctionWithDescriptor:error:"),
            descriptor.as_raw(),
            &mut err,
        );
        *error = Strong::from_transfer(err);
        Strong::from_transfer(function)
    }
}

// ---------------------------------------------------------------------------
// MTL::Function / MTL::VertexAttribute
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::Function` (`Metal/MTLLibrary.hpp`).
    pub struct MTLFunction : "MTLFunction";
);

impl MTLFunction {
    /// `-name`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }

    /// `-functionType`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn functionType(&self) -> MTLFunctionType {
        send::<MTLFunctionType>(self.as_raw(), sel!("functionType"))
    }

    /// `-patchType`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn patchType(&self) -> MTLPatchType {
        send::<MTLPatchType>(self.as_raw(), sel!("patchType"))
    }

    /// `-patchControlPointCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn patchControlPointCount(&self) -> i64 {
        send::<i64>(self.as_raw(), sel!("patchControlPointCount"))
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

    /// `-options` (`MTLOptions` bitmask, mapped as `MTLFunctionOptions`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn options(&self) -> MTLFunctionOptions {
        send::<MTLFunctionOptions>(self.as_raw(), sel!("options"))
    }

    /// `-vertexAttributes`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn vertexAttributes(&self) -> Ref<NSArray<MTLVertexAttribute>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("vertexAttributes")))
    }

    /// `-stageInputAttributes`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stageInputAttributes(&self) -> Ref<NSArray<MTLVertexAttribute>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("stageInputAttributes")))
    }

    /// `-functionConstantsDictionary`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn functionConstantsDictionary(&self) -> Ref<NSDictionary<NSString, NSObject>> {
        Ref::from_raw(send::<Raw>(
            self.as_raw(),
            sel!("functionConstantsDictionary"),
        ))
    }

    /// `-newArgumentEncoderWithBufferIndex:` (+1 → transferred).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn newArgumentEncoder(&self, bufferIndex: NSUInteger) -> Strong<MTLArgumentEncoder> {
        Strong::from_transfer(send1::<NSUInteger, Raw>(
            self.as_raw(),
            sel!("newArgumentEncoderWithBufferIndex:"),
            bufferIndex,
        ))
    }
}

objc_class!(
    /// `MTL::VertexAttribute` (`Metal/MTLLibrary.hpp`).
    pub struct MTLVertexAttribute : "MTLVertexAttribute";
);

impl MTLVertexAttribute {
    /// `-attributeIndex`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn attributeIndex(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("attributeIndex"))
    }

    /// `-attributeType`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn attributeType(&self) -> MTLDataType {
        send::<MTLDataType>(self.as_raw(), sel!("attributeType"))
    }

    /// `-name`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }

    /// `-isActive`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isActive(&self) -> bool {
        send_bool(self.as_raw(), sel!("isActive"))
    }
}

// ---------------------------------------------------------------------------
// MTL::CompileOptions
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::CompileOptions` (`Metal/MTLLibrary.hpp`).
    pub struct MTLCompileOptions : "MTLCompileOptions";
);

impl MTLCompileOptions {
    /// `-fastMathEnabled` / `-setFastMathEnabled:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fastMathEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("fastMathEnabled"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFastMathEnabled(&self, fastMathEnabled: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setFastMathEnabled:"),
            bool_arg(fastMathEnabled),
        );
    }

    /// `-languageVersion` / `-setLanguageVersion:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn languageVersion(&self) -> MTLLanguageVersion {
        send::<MTLLanguageVersion>(self.as_raw(), sel!("languageVersion"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLanguageVersion(&self, languageVersion: MTLLanguageVersion) {
        send1::<MTLLanguageVersion, ()>(
            self.as_raw(),
            sel!("setLanguageVersion:"),
            languageVersion,
        );
    }

    /// `-libraryType` / `-setLibraryType:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn libraryType(&self) -> MTLLibraryType {
        send::<MTLLibraryType>(self.as_raw(), sel!("libraryType"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setLibraryType(&self, libraryType: MTLLibraryType) {
        send1::<MTLLibraryType, ()>(self.as_raw(), sel!("setLibraryType:"), libraryType);
    }

    /// `-optimizationLevel` / `-setOptimizationLevel:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn optimizationLevel(&self) -> MTLLibraryOptimizationLevel {
        send::<MTLLibraryOptimizationLevel>(self.as_raw(), sel!("optimizationLevel"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setOptimizationLevel(&self, optimizationLevel: MTLLibraryOptimizationLevel) {
        send1::<MTLLibraryOptimizationLevel, ()>(
            self.as_raw(),
            sel!("setOptimizationLevel:"),
            optimizationLevel,
        );
    }

    /// `-mathMode` / `-setMathMode:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mathMode(&self) -> MTLMathMode {
        send::<MTLMathMode>(self.as_raw(), sel!("mathMode"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMathMode(&self, mathMode: MTLMathMode) {
        send1::<MTLMathMode, ()>(self.as_raw(), sel!("setMathMode:"), mathMode);
    }

    /// `-mathFloatingPointFunctions` / `-setMathFloatingPointFunctions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mathFloatingPointFunctions(&self) -> MTLMathFloatingPointFunctions {
        send::<MTLMathFloatingPointFunctions>(self.as_raw(), sel!("mathFloatingPointFunctions"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setMathFloatingPointFunctions(
        &self,
        mathFloatingPointFunctions: MTLMathFloatingPointFunctions,
    ) {
        send1::<MTLMathFloatingPointFunctions, ()>(
            self.as_raw(),
            sel!("setMathFloatingPointFunctions:"),
            mathFloatingPointFunctions,
        );
    }

    /// `-floatingPointConversionRoundingMode` / setter
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn floatingPointConversionRoundingMode(
        &self,
    ) -> MTLFloatingPointConversionRoundingMode {
        send::<MTLFloatingPointConversionRoundingMode>(
            self.as_raw(),
            sel!("floatingPointConversionRoundingMode"),
        )
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setFloatingPointConversionRoundingMode(
        &self,
        mode: MTLFloatingPointConversionRoundingMode,
    ) {
        send1::<MTLFloatingPointConversionRoundingMode, ()>(
            self.as_raw(),
            sel!("setFloatingPointConversionRoundingMode:"),
            mode,
        );
    }

    /// `-compileSymbolVisibility` / `-setCompileSymbolVisibility:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn compileSymbolVisibility(&self) -> MTLCompileSymbolVisibility {
        send::<MTLCompileSymbolVisibility>(self.as_raw(), sel!("compileSymbolVisibility"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setCompileSymbolVisibility(
        &self,
        compileSymbolVisibility: MTLCompileSymbolVisibility,
    ) {
        send1::<MTLCompileSymbolVisibility, ()>(
            self.as_raw(),
            sel!("setCompileSymbolVisibility:"),
            compileSymbolVisibility,
        );
    }

    /// `-preserveInvariance` / `-setPreserveInvariance:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn preserveInvariance(&self) -> bool {
        send_bool(self.as_raw(), sel!("preserveInvariance"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPreserveInvariance(&self, preserveInvariance: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setPreserveInvariance:"),
            bool_arg(preserveInvariance),
        );
    }

    /// `-allowReferencingUndefinedSymbols` / setter
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn allowReferencingUndefinedSymbols(&self) -> bool {
        send_bool(self.as_raw(), sel!("allowReferencingUndefinedSymbols"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setAllowReferencingUndefinedSymbols(&self, allow: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setAllowReferencingUndefinedSymbols:"),
            bool_arg(allow),
        );
    }

    /// `-enableLogging` / `-setEnableLogging:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn enableLogging(&self) -> bool {
        send_bool(self.as_raw(), sel!("enableLogging"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setEnableLogging(&self, enableLogging: bool) {
        send1::<crate::objc_abi::ObjBOOL, ()>(
            self.as_raw(),
            sel!("setEnableLogging:"),
            bool_arg(enableLogging),
        );
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
    pub unsafe fn setMaxTotalThreadsPerThreadgroup(
        &self,
        maxTotalThreadsPerThreadgroup: NSUInteger,
    ) {
        send1::<NSUInteger, ()>(
            self.as_raw(),
            sel!("setMaxTotalThreadsPerThreadgroup:"),
            maxTotalThreadsPerThreadgroup,
        );
    }

    /// `-preprocessorMacros` / `-setPreprocessorMacros:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn preprocessorMacros(&self) -> Ref<NSDictionary<NSString, NSObject>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("preprocessorMacros")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setPreprocessorMacros(
        &self,
        preprocessorMacros: Ref<NSDictionary<NSString, NSObject>>,
    ) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setPreprocessorMacros:"),
            preprocessorMacros.as_raw(),
        );
    }

    /// `-installName` / `-setInstallName:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn installName(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("installName")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setInstallName(&self, installName: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setInstallName:"), installName.as_raw());
    }

    /// `-requiredThreadsPerThreadgroup` / `-setRequiredThreadsPerThreadgroup:`
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
}

// ---------------------------------------------------------------------------
// MTL::FunctionDescriptor / MTL::FunctionConstantValues
// ---------------------------------------------------------------------------

objc_class!(
    /// `MTL::FunctionDescriptor` (`Metal/MTLFunctionDescriptor.hpp`).
    pub struct MTLFunctionDescriptor : "MTLFunctionDescriptor";
);

impl MTLFunctionDescriptor {
    /// `+[MTLFunctionDescriptor functionDescriptor]` (class factory, +0 → retained).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn functionDescriptor() -> Strong<MTLFunctionDescriptor> {
        Strong::from_retain(send::<Raw>(
            MTLFunctionDescriptor::class().ptr(),
            sel!("functionDescriptor"),
        ))
    }

    /// `-name` / `-setName:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setName(&self, name: Ref<NSString>) {
        send1::<Raw, ()>(self.as_raw(), sel!("setName:"), name.as_raw());
    }

    /// `-specializedName` / `-setSpecializedName:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn specializedName(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("specializedName")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setSpecializedName(&self, specializedName: Ref<NSString>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setSpecializedName:"),
            specializedName.as_raw(),
        );
    }

    /// `-options` / `-setOptions:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn options(&self) -> MTLFunctionOptions {
        send::<MTLFunctionOptions>(self.as_raw(), sel!("options"))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setOptions(&self, options: MTLFunctionOptions) {
        send1::<MTLFunctionOptions, ()>(self.as_raw(), sel!("setOptions:"), options);
    }

    /// `-constantValues` / `-setConstantValues:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn constantValues(&self) -> Ref<MTLFunctionConstantValues> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("constantValues")))
    }
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setConstantValues(&self, constantValues: Ref<MTLFunctionConstantValues>) {
        send1::<Raw, ()>(
            self.as_raw(),
            sel!("setConstantValues:"),
            constantValues.as_raw(),
        );
    }
}

objc_class!(
    /// `MTL::FunctionConstantValues` (`Metal/MTLFunctionConstantValues.hpp`).
    pub struct MTLFunctionConstantValues : "MTLFunctionConstantValues";
);

impl MTLFunctionConstantValues {
    /// `-reset`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn reset(&self) {
        send::<()>(self.as_raw(), sel!("reset"));
    }

    /// `-setConstantValue:type:atIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setConstantValue(
        &self,
        value: *const c_void,
        type_: MTLDataType,
        index: NSUInteger,
    ) {
        send3::<*const c_void, MTLDataType, NSUInteger, ()>(
            self.as_raw(),
            sel!("setConstantValue:type:atIndex:"),
            value,
            type_,
            index,
        );
    }

    /// `-setConstantValue:type:withName:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setConstantValueWithName(
        &self,
        value: *const c_void,
        type_: MTLDataType,
        name: Ref<NSString>,
    ) {
        send3::<*const c_void, MTLDataType, Raw, ()>(
            self.as_raw(),
            sel!("setConstantValue:type:withName:"),
            value,
            type_,
            name.as_raw(),
        );
    }

    /// `-setConstantValues:type:withRange:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setConstantValues(
        &self,
        values: *const c_void,
        type_: MTLDataType,
        range: crate::foundation::NSRange,
    ) {
        send3::<*const c_void, MTLDataType, crate::foundation::NSRange, ()>(
            self.as_raw(),
            sel!("setConstantValues:type:withRange:"),
            values,
            type_,
            range,
        );
    }
}
