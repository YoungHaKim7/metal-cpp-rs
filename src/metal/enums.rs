//! Metal enums and option masks — Rust mirrors of the `_MTL_ENUM` /
//! `MTL_OPTIONS` declarations in the metal-cpp headers, with **verbatim**
//! discriminant values (these are the ABI).
//!
//! Enumerators keep the prefixed names metal-cpp uses
//! (e.g. `MTLPixelFormat::PixelFormatBGRA8Unorm`, `MTL::TextureType::TextureType2D`).

#![allow(non_camel_case_types)]

use std::ops::{BitAnd, BitOr, BitOrAssign};

macro_rules! mtl_options {
    (
        $(#[$m:meta])*
        $name:ident : $backing:ty { $( $c:ident = $v:expr ),+ $(,)? }
    ) => {
        $(#[$m])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
        #[repr(transparent)]
        pub struct $name(pub $backing);

        impl $name {
            $( pub const $c: Self = Self($v); )+

            /// The raw ABI value.
            pub const fn raw(self) -> $backing {
                self.0
            }

            /// True if every bit of `other` is set in `self`.
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// `a | b`.
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }

        impl BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0;
            }
        }
        impl BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
    };
}

// ---------------------------------------------------------------------------
// MTLPixelFormat.hpp — complete (152 enumerators, values verbatim).
// ---------------------------------------------------------------------------

/// `MTL::PixelFormat` (`Metal/MTLPixelFormat.hpp`).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLPixelFormat {
    PixelFormatInvalid = 0,
    PixelFormatA8Unorm = 1,
    PixelFormatR8Unorm = 10,
    PixelFormatR8Unorm_sRGB = 11,
    PixelFormatR8Snorm = 12,
    PixelFormatR8Uint = 13,
    PixelFormatR8Sint = 14,
    PixelFormatR16Unorm = 20,
    PixelFormatR16Snorm = 22,
    PixelFormatR16Uint = 23,
    PixelFormatR16Sint = 24,
    PixelFormatR16Float = 25,
    PixelFormatRG8Unorm = 30,
    PixelFormatRG8Unorm_sRGB = 31,
    PixelFormatRG8Snorm = 32,
    PixelFormatRG8Uint = 33,
    PixelFormatRG8Sint = 34,
    PixelFormatB5G6R5Unorm = 40,
    PixelFormatA1BGR5Unorm = 41,
    PixelFormatABGR4Unorm = 42,
    PixelFormatBGR5A1Unorm = 43,
    PixelFormatRGB8Unorm = 45,
    PixelFormatRGB8Snorm = 46,
    PixelFormatRGB8Uint = 47,
    PixelFormatRGB8Sint = 48,
    PixelFormatR32Uint = 53,
    PixelFormatR32Sint = 54,
    PixelFormatR32Float = 55,
    PixelFormatRG16Unorm = 60,
    PixelFormatRG16Snorm = 62,
    PixelFormatRG16Uint = 63,
    PixelFormatRG16Sint = 64,
    PixelFormatRG16Float = 65,
    PixelFormatRGBA8Unorm = 70,
    PixelFormatRGBA8Unorm_sRGB = 71,
    PixelFormatRGBA8Snorm = 72,
    PixelFormatRGBA8Uint = 73,
    PixelFormatRGBA8Sint = 74,
    PixelFormatBGRA8Unorm = 80,
    PixelFormatBGRA8Unorm_sRGB = 81,
    PixelFormatRGB10A2Unorm = 90,
    PixelFormatRGB10A2Uint = 91,
    PixelFormatRG11B10Float = 92,
    PixelFormatRGB9E5Float = 93,
    PixelFormatBGR10A2Unorm = 94,
    PixelFormatRGB16Unorm = 95,
    PixelFormatRGB16Snorm = 96,
    PixelFormatRGB16Uint = 97,
    PixelFormatRGB16Sint = 98,
    PixelFormatRGB16Float = 99,
    PixelFormatRGBA16Unorm = 110,
    PixelFormatRGBA16Snorm = 112,
    PixelFormatRGBA16Uint = 113,
    PixelFormatRGBA16Sint = 114,
    PixelFormatRGBA16Float = 115,
    PixelFormatRGB32Uint = 120,
    PixelFormatRGB32Sint = 121,
    PixelFormatRGB32Float = 122,
    PixelFormatRGBA32Uint = 123,
    PixelFormatRGBA32Sint = 124,
    PixelFormatRGBA32Float = 125,
    PixelFormatGBGR422 = 240,
    PixelFormatBGRG422 = 241,
    PixelFormatDepth16Unorm = 250,
    PixelFormatDepth32Float = 252,
    PixelFormatStencil8 = 253,
    PixelFormatDepth24Unorm_Stencil8 = 255,
    PixelFormatX32_Stencil8 = 261,
    PixelFormatX24_Stencil8 = 262,
    PixelFormatDepth32Float_Stencil8 = 260,
    PixelFormatUnspecialized = 263,
    PixelFormatBC1_RGBA = 130,
    PixelFormatBC1_RGBA_sRGB = 131,
    PixelFormatBC2_RGBA = 132,
    PixelFormatBC2_RGBA_sRGB = 133,
    PixelFormatBC3_RGBA = 134,
    PixelFormatBC3_RGBA_sRGB = 135,
    PixelFormatBC4_RUnorm = 140,
    PixelFormatBC4_RSnorm = 141,
    PixelFormatBC5_RGUnorm = 142,
    PixelFormatBC5_RGSnorm = 143,
    PixelFormatBC6H_RGBFloat = 150,
    PixelFormatBC6H_RGBUfloat = 151,
    PixelFormatBC7_RGBAUnorm = 152,
    PixelFormatBC7_RGBAUnorm_sRGB = 153,
    PixelFormatPVRTC_RGB_2BPP = 160,
    PixelFormatPVRTC_RGB_2BPP_sRGB = 161,
    PixelFormatPVRTC_RGB_4BPP = 162,
    PixelFormatPVRTC_RGB_4BPP_sRGB = 163,
    PixelFormatPVRTC_RGBA_2BPP = 164,
    PixelFormatPVRTC_RGBA_2BPP_sRGB = 165,
    PixelFormatPVRTC_RGBA_4BPP = 166,
    PixelFormatPVRTC_RGBA_4BPP_sRGB = 167,
    PixelFormatEAC_R11Unorm = 170,
    PixelFormatEAC_R11Snorm = 172,
    PixelFormatEAC_RG11Unorm = 174,
    PixelFormatEAC_RG11Snorm = 176,
    PixelFormatEAC_RGBA8 = 178,
    PixelFormatEAC_RGBA8_sRGB = 179,
    PixelFormatETC2_RGB8 = 180,
    PixelFormatETC2_RGB8_sRGB = 181,
    PixelFormatETC2_RGB8A1 = 182,
    PixelFormatETC2_RGB8A1_sRGB = 183,
    PixelFormatASTC_4x4_sRGB = 186,
    PixelFormatASTC_5x4_sRGB = 187,
    PixelFormatASTC_5x5_sRGB = 188,
    PixelFormatASTC_6x5_sRGB = 189,
    PixelFormatASTC_6x6_sRGB = 190,
    PixelFormatASTC_8x5_sRGB = 192,
    PixelFormatASTC_8x6_sRGB = 193,
    PixelFormatASTC_8x8_sRGB = 194,
    PixelFormatASTC_10x5_sRGB = 195,
    PixelFormatASTC_10x6_sRGB = 196,
    PixelFormatASTC_10x8_sRGB = 197,
    PixelFormatASTC_10x10_sRGB = 198,
    PixelFormatASTC_12x10_sRGB = 199,
    PixelFormatASTC_12x12_sRGB = 200,
    PixelFormatASTC_4x4_LDR = 204,
    PixelFormatASTC_5x4_LDR = 205,
    PixelFormatASTC_5x5_LDR = 206,
    PixelFormatASTC_6x5_LDR = 207,
    PixelFormatASTC_6x6_LDR = 208,
    PixelFormatASTC_8x5_LDR = 210,
    PixelFormatASTC_8x6_LDR = 211,
    PixelFormatASTC_8x8_LDR = 212,
    PixelFormatASTC_10x5_LDR = 213,
    PixelFormatASTC_10x6_LDR = 214,
    PixelFormatASTC_10x8_LDR = 215,
    PixelFormatASTC_10x10_LDR = 216,
    PixelFormatASTC_12x10_LDR = 217,
    PixelFormatASTC_12x12_LDR = 218,
    PixelFormatASTC_4x4_HDR = 222,
    PixelFormatASTC_5x4_HDR = 223,
    PixelFormatASTC_5x5_HDR = 224,
    PixelFormatASTC_6x5_HDR = 225,
    PixelFormatASTC_6x6_HDR = 226,
    PixelFormatASTC_8x5_HDR = 228,
    PixelFormatASTC_8x6_HDR = 229,
    PixelFormatASTC_8x8_HDR = 230,
    PixelFormatASTC_10x5_HDR = 231,
    PixelFormatASTC_10x6_HDR = 232,
    PixelFormatASTC_10x8_HDR = 233,
    PixelFormatASTC_10x10_HDR = 234,
    PixelFormatASTC_12x10_HDR = 235,
    PixelFormatASTC_12x12_HDR = 236,
    PixelFormatBGRA10_XR = 552,
    PixelFormatBGRA10_XR_sRGB = 553,
    PixelFormatBGR10_XR = 554,
    PixelFormatBGR10_XR_sRGB = 555,
}

// ---------------------------------------------------------------------------
// MTLResource.hpp
// ---------------------------------------------------------------------------

/// `MTL::StorageMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLStorageMode {
    StorageModeShared = 0,
    StorageModeManaged = 1,
    StorageModePrivate = 2,
    StorageModeMemoryless = 3,
}

/// `MTL::CPUCacheMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCPUCacheMode {
    CPUCacheModeDefaultCache = 0,
    CPUCacheModeWriteCombined = 1,
}

/// `MTL::HazardTrackingMode` (current-metal-cpp order: Default/Untracked/Tracked).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLHazardTrackingMode {
    HazardTrackingModeDefault = 0,
    HazardTrackingModeUntracked = 1,
    HazardTrackingModeTracked = 2,
}

/// `MTL::PurgeableState`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLPurgeableState {
    PurgeableStateKeepCurrent = 1,
    PurgeableStateNonVolatile = 2,
    PurgeableStateVolatile = 3,
    PurgeableStateEmpty = 4,
}

/// `MTL::SparsePageSize` (`NS::Integer`-backed).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSparsePageSize {
    SparsePageSize16 = 101,
    SparsePageSize64 = 102,
    SparsePageSize256 = 103,
}

/// `MTL::BufferSparseTier`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBufferSparseTier {
    BufferSparseTierNone = 0,
    BufferSparseTier1 = 1,
}

/// `MTL::TextureSparseTier`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTextureSparseTier {
    TextureSparseTierNone = 0,
    TextureSparseTier1 = 1,
    TextureSparseTier2 = 2,
}

mtl_options! {
    /// `MTL::ResourceOptions` — packed storage/cache/hazard bits, values verbatim.
    MTLResourceOptions : u64 {
        ResourceCPUCacheModeDefaultCache = 0x0,
        ResourceCPUCacheModeWriteCombined = 0x1,
        ResourceStorageModeShared = 0x0,
        ResourceStorageModeManaged = 0x10,
        ResourceStorageModePrivate = 0x20,
        ResourceStorageModeMemoryless = 0x30,
        ResourceHazardTrackingModeDefault = 0x0,
        ResourceHazardTrackingModeUntracked = 0x100,
        ResourceHazardTrackingModeTracked = 0x200,
    }
}

// ---------------------------------------------------------------------------
// MTLTexture.hpp
// ---------------------------------------------------------------------------

/// `MTL::TextureType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTextureType {
    TextureType1D = 0,
    TextureType1DArray = 1,
    TextureType2D = 2,
    TextureType2DArray = 3,
    TextureType2DMultisample = 4,
    TextureTypeCube = 5,
    TextureTypeCubeArray = 6,
    TextureType3D = 7,
    TextureType2DMultisampleArray = 8,
    TextureTypeTextureBuffer = 9,
}

/// `MTL::TextureSwizzle` (`uint8_t`-backed).
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTextureSwizzle {
    TextureSwizzleZero = 0,
    TextureSwizzleOne = 1,
    TextureSwizzleRed = 2,
    TextureSwizzleGreen = 3,
    TextureSwizzleBlue = 4,
    TextureSwizzleAlpha = 5,
}

/// `MTL::TextureCompressionType`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTextureCompressionType {
    TextureCompressionTypeLossless = 0,
    TextureCompressionTypeLossy = 1,
}

mtl_options! {
    /// `MTL::TextureUsage` — values verbatim (bit 3 is unused upstream).
    MTLTextureUsage : u64 {
        TextureUsageUnknown = 0,
        TextureUsageShaderRead = 1,
        TextureUsageShaderWrite = 1 << 1,
        TextureUsageRenderTarget = 1 << 2,
        TextureUsagePixelFormatView = 1 << 4,
        TextureUsageShaderAtomic = 1 << 5,
    }
}

// ---------------------------------------------------------------------------
// MTLSampler.hpp
// ---------------------------------------------------------------------------

/// `MTL::SamplerMinMagFilter`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSamplerMinMagFilter {
    SamplerMinMagFilterNearest = 0,
    SamplerMinMagFilterLinear = 1,
}

/// `MTL::SamplerMipFilter`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSamplerMipFilter {
    SamplerMipFilterNotMipmapped = 0,
    SamplerMipFilterNearest = 1,
    SamplerMipFilterLinear = 2,
}

/// `MTL::SamplerAddressMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSamplerAddressMode {
    SamplerAddressModeClampToEdge = 0,
    SamplerAddressModeMirrorClampToEdge = 1,
    SamplerAddressModeRepeat = 2,
    SamplerAddressModeMirrorRepeat = 3,
    SamplerAddressModeClampToZero = 4,
    SamplerAddressModeClampToBorderColor = 5,
}

/// `MTL::SamplerBorderColor`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSamplerBorderColor {
    SamplerBorderColorTransparentBlack = 0,
    SamplerBorderColorOpaqueBlack = 1,
    SamplerBorderColorOpaqueWhite = 2,
}

/// `MTL::SamplerReductionMode`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLSamplerReductionMode {
    SamplerReductionModeWeightedAverage = 0,
    SamplerReductionModeMinimum = 1,
    SamplerReductionModeMaximum = 2,
}

// ---------------------------------------------------------------------------
// MTLRenderPass.hpp
// ---------------------------------------------------------------------------

/// `MTL::LoadAction`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLLoadAction {
    LoadActionDontCare = 0,
    LoadActionLoad = 1,
    LoadActionClear = 2,
}

/// `MTL::StoreAction`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLStoreAction {
    StoreActionDontCare = 0,
    StoreActionStore = 1,
    StoreActionMultisampleResolve = 2,
    StoreActionStoreAndMultisampleResolve = 3,
    StoreActionUnknown = 4,
    StoreActionCustomSampleDepthStore = 5,
}

mtl_options! {
    /// `MTL::StoreActionOptions`.
    MTLStoreActionOptions : u64 {
        StoreActionOptionNone = 0,
        StoreActionOptionCustomSamplePositions = 1,
        StoreActionOptionValidMask = 1,
    }
}

/// `MTL::VisibilityResultType`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLVisibilityResultType {
    VisibilityResultTypeReset = 0,
    VisibilityResultTypeAccumulate = 1,
}

/// `MTL::MultisampleDepthResolveFilter` (current-metal-cpp: Sample0/Min/Max).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLMultisampleDepthResolveFilter {
    MultisampleDepthResolveFilterSample0 = 0,
    MultisampleDepthResolveFilterMin = 1,
    MultisampleDepthResolveFilterMax = 2,
}

/// `MTL::MultisampleStencilResolveFilter`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLMultisampleStencilResolveFilter {
    MultisampleStencilResolveFilterSample0 = 0,
    MultisampleStencilResolveFilterDepthResolvedSample = 1,
}

// ---------------------------------------------------------------------------
// MTLDepthStencil.hpp
// ---------------------------------------------------------------------------

/// `MTL::CompareFunction`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCompareFunction {
    CompareFunctionNever = 0,
    CompareFunctionLess = 1,
    CompareFunctionEqual = 2,
    CompareFunctionLessEqual = 3,
    CompareFunctionGreater = 4,
    CompareFunctionNotEqual = 5,
    CompareFunctionGreaterEqual = 6,
    CompareFunctionAlways = 7,
}

/// `MTL::StencilOperation`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLStencilOperation {
    StencilOperationKeep = 0,
    StencilOperationZero = 1,
    StencilOperationReplace = 2,
    StencilOperationIncrementClamp = 3,
    StencilOperationDecrementClamp = 4,
    StencilOperationInvert = 5,
    StencilOperationIncrementWrap = 6,
    StencilOperationDecrementWrap = 7,
}

// ---------------------------------------------------------------------------
// MTLRenderPipeline.hpp (+ MTLBlendFactor / MTLBlendOperation live here)
// ---------------------------------------------------------------------------

/// `MTL::BlendFactor`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBlendFactor {
    BlendFactorZero = 0,
    BlendFactorOne = 1,
    BlendFactorSourceColor = 2,
    BlendFactorOneMinusSourceColor = 3,
    BlendFactorSourceAlpha = 4,
    BlendFactorOneMinusSourceAlpha = 5,
    BlendFactorDestinationColor = 6,
    BlendFactorOneMinusDestinationColor = 7,
    BlendFactorDestinationAlpha = 8,
    BlendFactorOneMinusDestinationAlpha = 9,
    BlendFactorSourceAlphaSaturated = 10,
    BlendFactorBlendColor = 11,
    BlendFactorOneMinusBlendColor = 12,
    BlendFactorBlendAlpha = 13,
    BlendFactorOneMinusBlendAlpha = 14,
    BlendFactorSource1Color = 15,
    BlendFactorOneMinusSource1Color = 16,
    BlendFactorSource1Alpha = 17,
    BlendFactorOneMinusSource1Alpha = 18,
    BlendFactorUnspecialized = 19,
}

/// `MTL::BlendOperation`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBlendOperation {
    BlendOperationAdd = 0,
    BlendOperationSubtract = 1,
    BlendOperationReverseSubtract = 2,
    BlendOperationMin = 3,
    BlendOperationMax = 4,
    BlendOperationUnspecialized = 5,
}

mtl_options! {
    /// `MTL::ColorWriteMask` — note the upstream bit order: Red is bit 3!
    MTLColorWriteMask : u64 {
        ColorWriteMaskNone = 0,
        ColorWriteMaskRed = 1 << 3,
        ColorWriteMaskGreen = 1 << 2,
        ColorWriteMaskBlue = 1 << 1,
        ColorWriteMaskAlpha = 1,
        ColorWriteMaskAll = 15,
        ColorWriteMaskUnspecialized = 1 << 4,
    }
}

/// `MTL::PrimitiveTopologyClass`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLPrimitiveTopologyClass {
    PrimitiveTopologyClassUnspecified = 0,
    PrimitiveTopologyClassPoint = 1,
    PrimitiveTopologyClassLine = 2,
    PrimitiveTopologyClassTriangle = 3,
}

/// `MTL::TessellationPartitionMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTessellationPartitionMode {
    TessellationPartitionModePow2 = 0,
    TessellationPartitionModeInteger = 1,
    TessellationPartitionModeFractionalOdd = 2,
    TessellationPartitionModeFractionalEven = 3,
}

/// `MTL::TessellationFactorStepFunction`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTessellationFactorStepFunction {
    TessellationFactorStepFunctionConstant = 0,
    TessellationFactorStepFunctionPerPatch = 1,
    TessellationFactorStepFunctionPerInstance = 2,
    TessellationFactorStepFunctionPerPatchAndPerInstance = 3,
}

/// `MTL::TessellationFactorFormat`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTessellationFactorFormat {
    TessellationFactorFormatHalf = 0,
}

/// `MTL::TessellationControlPointIndexType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTessellationControlPointIndexType {
    TessellationControlPointIndexTypeNone = 0,
    TessellationControlPointIndexTypeUInt16 = 1,
    TessellationControlPointIndexTypeUInt32 = 2,
}

// ---------------------------------------------------------------------------
// MTLPipeline.hpp — ShaderValidation / Mutability / ForwardProgressUsage /
// MTLTypes.hpp — ContentionRelief
// ---------------------------------------------------------------------------

/// `MTL::ShaderValidation`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLShaderValidation {
    ShaderValidationDefault = 0,
    ShaderValidationEnabled = 1,
    ShaderValidationDisabled = 2,
}

/// `MTL::Mutability`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLMutability {
    MutabilityDefault = 0,
    MutabilityMutable = 1,
    MutabilityImmutable = 2,
}

/// `MTL::ForwardProgressUsage`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLForwardProgressUsage {
    ForwardProgressUsageAutomatic = 0,
    ForwardProgressUsageWeak = 1,
    ForwardProgressUsageSIMDGroupParallel = 2,
}

/// `MTL::ContentionRelief` (`MTLTypes.hpp`, `NS::Integer`-backed).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLContentionRelief {
    ContentionReliefAutomatic = 0,
    ContentionReliefNone = 1,
}

mtl_options! {
    /// `MTL::PipelineOption` (values verbatim; ArgumentInfo == BindingInfo == 1).
    MTLPipelineOption : u64 {
        PipelineOptionNone = 0,
        PipelineOptionArgumentInfo = 1,
        PipelineOptionBindingInfo = 1,
        PipelineOptionBufferTypeInfo = 1 << 1,
        PipelineOptionFailOnBinaryArchiveMiss = 1 << 2,
    }
}

// ---------------------------------------------------------------------------
// MTLLibrary.hpp
// ---------------------------------------------------------------------------

/// `MTL::FunctionType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLFunctionType {
    FunctionTypeVertex = 1,
    FunctionTypeFragment = 2,
    FunctionTypeKernel = 3,
    FunctionTypeVisible = 5,
    FunctionTypeIntersection = 6,
    FunctionTypeMesh = 7,
    FunctionTypeObject = 8,
}

/// `MTL::PatchType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLPatchType {
    PatchTypeNone = 0,
    PatchTypeTriangle = 1,
    PatchTypeQuad = 2,
}

/// `MTL::LanguageVersion` — packed `major << 16 | minor`, values verbatim.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLLanguageVersion {
    LanguageVersion1_0 = 65536,
    LanguageVersion1_1 = 65537,
    LanguageVersion1_2 = 65538,
    LanguageVersion2_0 = 131072,
    LanguageVersion2_1 = 131073,
    LanguageVersion2_2 = 131074,
    LanguageVersion2_3 = 131075,
    LanguageVersion2_4 = 131076,
    LanguageVersion3_0 = 196608,
    LanguageVersion3_1 = 196609,
    LanguageVersion3_2 = 196610,
    LanguageVersion4_0 = 262144,
    LanguageVersion4_1 = 262145,
}

/// `MTL::LibraryType`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLLibraryType {
    LibraryTypeExecutable = 0,
    LibraryTypeDynamic = 1,
}

/// `MTL::LibraryOptimizationLevel`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLLibraryOptimizationLevel {
    LibraryOptimizationLevelDefault = 0,
    LibraryOptimizationLevelSize = 1,
}

/// `MTL::CompileSymbolVisibility`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCompileSymbolVisibility {
    CompileSymbolVisibilityDefault = 0,
    CompileSymbolVisibilityHidden = 1,
}

/// `MTL::MathMode`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLMathMode {
    MathModeSafe = 0,
    MathModeRelaxed = 1,
    MathModeFast = 2,
}

/// `MTL::MathFloatingPointFunctions`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLMathFloatingPointFunctions {
    MathFloatingPointFunctionsFast = 0,
    MathFloatingPointFunctionsPrecise = 1,
}

/// `MTL::FloatingPointConversionRoundingMode`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLFloatingPointConversionRoundingMode {
    FloatingPointConversionRoundingModeToNearestEven = 0,
    FloatingPointConversionRoundingModeTowardZero = 1,
}

/// `MTL::LibraryError`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLLibraryError {
    LibraryErrorUnsupported = 1,
    LibraryErrorInternal = 2,
    LibraryErrorCompileFailure = 3,
    LibraryErrorCompileWarning = 4,
    LibraryErrorFunctionNotFound = 5,
    LibraryErrorFileNotFound = 6,
}

mtl_options! {
    /// `MTL::FunctionOptions` (`MTLFunctionDescriptor.hpp`).
    MTLFunctionOptions : u64 {
        FunctionOptionsNone = 0,
        FunctionOptionsCompileToBinary = 1,
        FunctionOptionsStoreFunctionInMetalPipelinesScript = 1 << 1,
        FunctionOptionsStoreFunctionInMetalScript = 1 << 1,
        FunctionOptionsFailOnBinaryArchiveMiss = 1 << 2,
        FunctionOptionsPipelineIndependent = 1 << 3,
    }
}

// ---------------------------------------------------------------------------
// MTLHeap.hpp
// ---------------------------------------------------------------------------

/// `MTL::HeapType`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLHeapType {
    HeapTypeAutomatic = 0,
    HeapTypePlacement = 1,
    HeapTypeSparse = 2,
}

// ---------------------------------------------------------------------------
// MTLDevice.hpp
// ---------------------------------------------------------------------------

/// `MTL::GPUFamily` (`NS::Integer`-backed; values verbatim incl. Apple11/Metal4).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLGPUFamily {
    GPUFamilyApple1 = 1001,
    GPUFamilyApple2 = 1002,
    GPUFamilyApple3 = 1003,
    GPUFamilyApple4 = 1004,
    GPUFamilyApple5 = 1005,
    GPUFamilyApple6 = 1006,
    GPUFamilyApple7 = 1007,
    GPUFamilyApple8 = 1008,
    GPUFamilyApple9 = 1009,
    GPUFamilyApple10 = 1010,
    GPUFamilyApple11 = 1011,
    GPUFamilyMac1 = 2001,
    GPUFamilyMac2 = 2002,
    GPUFamilyCommon1 = 3001,
    GPUFamilyCommon2 = 3002,
    GPUFamilyCommon3 = 3003,
    GPUFamilyMacCatalyst1 = 4001,
    GPUFamilyMacCatalyst2 = 4002,
    GPUFamilyMetal3 = 5001,
    GPUFamilyMetal4 = 5002,
}

/// `MTL::DeviceLocation` (`Unspecified` is `NS::UIntegerMax`).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLDeviceLocation {
    DeviceLocationBuiltIn = 0,
    DeviceLocationSlot = 1,
    DeviceLocationExternal = 2,
    DeviceLocationUnspecified = u64::MAX,
}

/// `MTL::ReadWriteTextureTier`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLReadWriteTextureTier {
    ReadWriteTextureTierNone = 0,
    ReadWriteTextureTier1 = 1,
    ReadWriteTextureTier2 = 2,
}

/// `MTL::ArgumentBuffersTier`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLArgumentBuffersTier {
    ArgumentBuffersTierTier1 = 0,
    ArgumentBuffersTierTier2 = 1,
}

/// `MTL::CounterSamplingPoint`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCounterSamplingPoint {
    CounterSamplingPointAtStageBoundary = 0,
    CounterSamplingPointAtDrawBoundary = 1,
    CounterSamplingPointAtDispatchBoundary = 2,
    CounterSamplingPointAtTileDispatchBoundary = 3,
    CounterSamplingPointAtBlitBoundary = 4,
}

// ---------------------------------------------------------------------------
// MTLArgument.hpp
// ---------------------------------------------------------------------------

/// `MTL::IndexType` (also used by render encoders).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLIndexType {
    IndexTypeUInt16 = 0,
    IndexTypeUInt32 = 1,
}

/// `MTL::ArgumentType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLArgumentType {
    ArgumentTypeBuffer = 0,
    ArgumentTypeThreadgroupMemory = 1,
    ArgumentTypeTexture = 2,
    ArgumentTypeSampler = 3,
    ArgumentTypeImageblockData = 16,
    ArgumentTypeImageblock = 17,
    ArgumentTypeVisibleFunctionTable = 24,
    ArgumentTypePrimitiveAccelerationStructure = 25,
    ArgumentTypeInstanceAccelerationStructure = 26,
    ArgumentTypeIntersectionFunctionTable = 27,
}

/// `MTL::BindingAccess` (aka `ArgumentAccess`).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBindingAccess {
    BindingAccessReadOnly = 0,
    BindingAccessReadWrite = 1,
    BindingAccessWriteOnly = 2,
}

/// `MTL::BindingType` (`NS::Integer`-backed).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBindingType {
    BindingTypeBuffer = 0,
    BindingTypeThreadgroupMemory = 1,
    BindingTypeTexture = 2,
    BindingTypeSampler = 3,
    BindingTypeImageblockData = 16,
    BindingTypeImageblock = 17,
    BindingTypeVisibleFunctionTable = 24,
    BindingTypePrimitiveAccelerationStructure = 25,
    BindingTypeInstanceAccelerationStructure = 26,
    BindingTypeIntersectionFunctionTable = 27,
    BindingTypeObjectPayload = 34,
    BindingTypeTensor = 37,
}

// ---------------------------------------------------------------------------
// MTLStageInputOutputDescriptor.hpp / MTLVertexDescriptor.hpp
// ---------------------------------------------------------------------------

/// `MTL::VertexFormat` (54 values, verbatim).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLVertexFormat {
    VertexFormatInvalid = 0,
    VertexFormatUChar2 = 1,
    VertexFormatUChar3 = 2,
    VertexFormatUChar4 = 3,
    VertexFormatChar2 = 4,
    VertexFormatChar3 = 5,
    VertexFormatChar4 = 6,
    VertexFormatUChar2Normalized = 7,
    VertexFormatUChar3Normalized = 8,
    VertexFormatUChar4Normalized = 9,
    VertexFormatChar2Normalized = 10,
    VertexFormatChar3Normalized = 11,
    VertexFormatChar4Normalized = 12,
    VertexFormatUShort2 = 13,
    VertexFormatUShort3 = 14,
    VertexFormatUShort4 = 15,
    VertexFormatShort2 = 16,
    VertexFormatShort3 = 17,
    VertexFormatShort4 = 18,
    VertexFormatUShort2Normalized = 19,
    VertexFormatUShort3Normalized = 20,
    VertexFormatUShort4Normalized = 21,
    VertexFormatShort2Normalized = 22,
    VertexFormatShort3Normalized = 23,
    VertexFormatShort4Normalized = 24,
    VertexFormatHalf2 = 25,
    VertexFormatHalf3 = 26,
    VertexFormatHalf4 = 27,
    VertexFormatFloat = 28,
    VertexFormatFloat2 = 29,
    VertexFormatFloat3 = 30,
    VertexFormatFloat4 = 31,
    VertexFormatInt = 32,
    VertexFormatInt2 = 33,
    VertexFormatInt3 = 34,
    VertexFormatInt4 = 35,
    VertexFormatUInt = 36,
    VertexFormatUInt2 = 37,
    VertexFormatUInt3 = 38,
    VertexFormatUInt4 = 39,
    VertexFormatInt1010102Normalized = 40,
    VertexFormatUInt1010102Normalized = 41,
    VertexFormatUChar4Normalized_BGRA = 42,
    VertexFormatUChar = 45,
    VertexFormatChar = 46,
    VertexFormatUCharNormalized = 47,
    VertexFormatCharNormalized = 48,
    VertexFormatUShort = 49,
    VertexFormatShort = 50,
    VertexFormatUShortNormalized = 51,
    VertexFormatShortNormalized = 52,
    VertexFormatHalf = 53,
    VertexFormatFloatRG11B10 = 54,
    VertexFormatFloatRGB9E5 = 55,
}

/// `MTL::AttributeFormat` (`MTLStageInputOutputDescriptor.hpp`) — same value
/// table as `VertexFormat`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLAttributeFormat {
    AttributeFormatInvalid = 0,
    AttributeFormatUChar2 = 1,
    AttributeFormatUChar3 = 2,
    AttributeFormatUChar4 = 3,
    AttributeFormatChar2 = 4,
    AttributeFormatChar3 = 5,
    AttributeFormatChar4 = 6,
    AttributeFormatUChar2Normalized = 7,
    AttributeFormatUChar3Normalized = 8,
    AttributeFormatUChar4Normalized = 9,
    AttributeFormatChar2Normalized = 10,
    AttributeFormatChar3Normalized = 11,
    AttributeFormatChar4Normalized = 12,
    AttributeFormatUShort2 = 13,
    AttributeFormatUShort3 = 14,
    AttributeFormatUShort4 = 15,
    AttributeFormatShort2 = 16,
    AttributeFormatShort3 = 17,
    AttributeFormatShort4 = 18,
    AttributeFormatUShort2Normalized = 19,
    AttributeFormatUShort3Normalized = 20,
    AttributeFormatUShort4Normalized = 21,
    AttributeFormatShort2Normalized = 22,
    AttributeFormatShort3Normalized = 23,
    AttributeFormatShort4Normalized = 24,
    AttributeFormatHalf2 = 25,
    AttributeFormatHalf3 = 26,
    AttributeFormatHalf4 = 27,
    AttributeFormatFloat = 28,
    AttributeFormatFloat2 = 29,
    AttributeFormatFloat3 = 30,
    AttributeFormatFloat4 = 31,
    AttributeFormatInt = 32,
    AttributeFormatInt2 = 33,
    AttributeFormatInt3 = 34,
    AttributeFormatInt4 = 35,
    AttributeFormatUInt = 36,
    AttributeFormatUInt2 = 37,
    AttributeFormatUInt3 = 38,
    AttributeFormatUInt4 = 39,
    AttributeFormatInt1010102Normalized = 40,
    AttributeFormatUInt1010102Normalized = 41,
    AttributeFormatUChar4Normalized_BGRA = 42,
    AttributeFormatUChar = 45,
    AttributeFormatChar = 46,
    AttributeFormatUCharNormalized = 47,
    AttributeFormatCharNormalized = 48,
    AttributeFormatUShort = 49,
    AttributeFormatShort = 50,
    AttributeFormatUShortNormalized = 51,
    AttributeFormatShortNormalized = 52,
    AttributeFormatHalf = 53,
    AttributeFormatFloatRG11B10 = 54,
    AttributeFormatFloatRGB9E5 = 55,
}

/// `MTL::VertexStepFunction`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLVertexStepFunction {
    VertexStepFunctionConstant = 0,
    VertexStepFunctionPerVertex = 1,
    VertexStepFunctionPerInstance = 2,
    VertexStepFunctionPerPatch = 3,
    VertexStepFunctionPerPatchControlPoint = 4,
}

/// `MTL::StepFunction` (`MTLStageInputOutputDescriptor.hpp`).
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLStepFunction {
    StepFunctionConstant = 0,
    StepFunctionPerVertex = 1,
    StepFunctionPerInstance = 2,
    StepFunctionPerPatch = 3,
    StepFunctionPerPatchControlPoint = 4,
    StepFunctionThreadPositionInGridX = 5,
    StepFunctionThreadPositionInGridY = 6,
    StepFunctionThreadPositionInGridXIndexed = 7,
    StepFunctionThreadPositionInGridYIndexed = 8,
}

// ---------------------------------------------------------------------------
// MTLDataType.hpp — complete (97 values, verbatim).
// ---------------------------------------------------------------------------

/// `MTL::DataType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLDataType {
    DataTypeNone = 0,
    DataTypeStruct = 1,
    DataTypeArray = 2,
    DataTypeFloat = 3,
    DataTypeFloat2 = 4,
    DataTypeFloat3 = 5,
    DataTypeFloat4 = 6,
    DataTypeFloat2x2 = 7,
    DataTypeFloat2x3 = 8,
    DataTypeFloat2x4 = 9,
    DataTypeFloat3x2 = 10,
    DataTypeFloat3x3 = 11,
    DataTypeFloat3x4 = 12,
    DataTypeFloat4x2 = 13,
    DataTypeFloat4x3 = 14,
    DataTypeFloat4x4 = 15,
    DataTypeHalf = 16,
    DataTypeHalf2 = 17,
    DataTypeHalf3 = 18,
    DataTypeHalf4 = 19,
    DataTypeHalf2x2 = 20,
    DataTypeHalf2x3 = 21,
    DataTypeHalf2x4 = 22,
    DataTypeHalf3x2 = 23,
    DataTypeHalf3x3 = 24,
    DataTypeHalf3x4 = 25,
    DataTypeHalf4x2 = 26,
    DataTypeHalf4x3 = 27,
    DataTypeHalf4x4 = 28,
    DataTypeInt = 29,
    DataTypeInt2 = 30,
    DataTypeInt3 = 31,
    DataTypeInt4 = 32,
    DataTypeUInt = 33,
    DataTypeUInt2 = 34,
    DataTypeUInt3 = 35,
    DataTypeUInt4 = 36,
    DataTypeShort = 37,
    DataTypeShort2 = 38,
    DataTypeShort3 = 39,
    DataTypeShort4 = 40,
    DataTypeUShort = 41,
    DataTypeUShort2 = 42,
    DataTypeUShort3 = 43,
    DataTypeUShort4 = 44,
    DataTypeChar = 45,
    DataTypeChar2 = 46,
    DataTypeChar3 = 47,
    DataTypeChar4 = 48,
    DataTypeUChar = 49,
    DataTypeUChar2 = 50,
    DataTypeUChar3 = 51,
    DataTypeUChar4 = 52,
    DataTypeBool = 53,
    DataTypeBool2 = 54,
    DataTypeBool3 = 55,
    DataTypeBool4 = 56,
    DataTypeTexture = 58,
    DataTypeSampler = 59,
    DataTypePointer = 60,
    DataTypeR8Unorm = 62,
    DataTypeR8Snorm = 63,
    DataTypeR16Unorm = 64,
    DataTypeR16Snorm = 65,
    DataTypeRG8Unorm = 66,
    DataTypeRG8Snorm = 67,
    DataTypeRG16Unorm = 68,
    DataTypeRG16Snorm = 69,
    DataTypeRGBA8Unorm = 70,
    DataTypeRGBA8Unorm_sRGB = 71,
    DataTypeRGBA8Snorm = 72,
    DataTypeRGBA16Unorm = 73,
    DataTypeRGBA16Snorm = 74,
    DataTypeRGB10A2Unorm = 75,
    DataTypeRG11B10Float = 76,
    DataTypeRGB9E5Float = 77,
    DataTypeRenderPipeline = 78,
    DataTypeComputePipeline = 79,
    DataTypeIndirectCommandBuffer = 80,
    DataTypeLong = 81,
    DataTypeLong2 = 82,
    DataTypeLong3 = 83,
    DataTypeLong4 = 84,
    DataTypeULong = 85,
    DataTypeULong2 = 86,
    DataTypeULong3 = 87,
    DataTypeULong4 = 88,
    DataTypeVisibleFunctionTable = 115,
    DataTypeIntersectionFunctionTable = 116,
    DataTypePrimitiveAccelerationStructure = 117,
    DataTypeInstanceAccelerationStructure = 118,
    DataTypeBFloat = 121,
    DataTypeBFloat2 = 122,
    DataTypeBFloat3 = 123,
    DataTypeBFloat4 = 124,
    DataTypeDepthStencilState = 139,
    DataTypeTensor = 140,
}

// ---------------------------------------------------------------------------
// MTLCommandBuffer.hpp / MTLCommandEncoder.hpp
// ---------------------------------------------------------------------------

/// `MTL::CommandBufferStatus`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCommandBufferStatus {
    CommandBufferStatusNotEnqueued = 0,
    CommandBufferStatusEnqueued = 1,
    CommandBufferStatusCommitted = 2,
    CommandBufferStatusScheduled = 3,
    CommandBufferStatusCompleted = 4,
    CommandBufferStatusError = 5,
}

/// `MTL::CommandBufferError`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCommandBufferError {
    CommandBufferErrorNone = 0,
    CommandBufferErrorInternal = 1,
    CommandBufferErrorTimeout = 2,
    CommandBufferErrorPageFault = 3,
    CommandBufferErrorBlacklisted = 4,
    CommandBufferErrorNotPermitted = 7,
    CommandBufferErrorOutOfMemory = 8,
    CommandBufferErrorInvalidResource = 9,
    CommandBufferErrorMemoryless = 10,
    CommandBufferErrorDeviceRemoved = 11,
    CommandBufferErrorStackOverflow = 12,
}

impl MTLCommandBufferError {
    /// Alias: `AccessRevoked` shares discriminant 4 with `Blacklisted`.
    pub const CommandBufferErrorAccessRevoked: MTLCommandBufferError =
        MTLCommandBufferError::CommandBufferErrorBlacklisted;
}

/// `MTL::CommandEncoderErrorState`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCommandEncoderErrorState {
    CommandEncoderErrorStateUnknown = 0,
    CommandEncoderErrorStateCompleted = 1,
    CommandEncoderErrorStateAffected = 2,
    CommandEncoderErrorStatePending = 3,
    CommandEncoderErrorStateFaulted = 4,
}

/// `MTL::DispatchType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLDispatchType {
    DispatchTypeSerial = 0,
    DispatchTypeConcurrent = 1,
}

mtl_options! {
    /// `MTL::CommandBufferErrorOption`.
    MTLCommandBufferErrorOption : u64 {
        CommandBufferErrorOptionNone = 0,
        CommandBufferErrorOptionEncoderExecutionStatus = 1,
    }
}

mtl_options! {
    /// `MTL::ResourceUsage` (passed to `useResource:usage:`).
    MTLResourceUsage : u64 {
        ResourceUsageRead = 1,
        ResourceUsageWrite = 1 << 1,
        ResourceUsageSample = 1 << 2,
    }
}

mtl_options! {
    /// `MTL::BarrierScope`.
    MTLBarrierScope : u64 {
        BarrierScopeBuffers = 1,
        BarrierScopeTextures = 1 << 1,
        BarrierScopeRenderTargets = 1 << 2,
    }
}

mtl_options! {
    /// `MTL::RenderStages` (render-encoder stage mask).
    MTLRenderStages : u64 {
        RenderStageVertex = 1,
        RenderStageFragment = 1 << 1,
        RenderStageTile = 1 << 2,
        RenderStageObject = 1 << 3,
        RenderStageMesh = 1 << 4,
    }
}

mtl_options! {
    /// `MTL::Stages` (queue-stage mask used by `barrierAfterQueueStages:beforeStages:`).
    MTLStages : u64 {
        StageVertex = 1,
        StageFragment = 1 << 1,
        StageTile = 1 << 2,
        StageObject = 1 << 3,
        StageMesh = 1 << 4,
        StageResourceState = 1 << 26,
        StageDispatch = 1 << 27,
        StageBlit = 1 << 28,
        StageAccelerationStructure = 1 << 29,
        StageMachineLearning = 1 << 30,
        StageAll = 9223372036854775807,
    }
}

// ---------------------------------------------------------------------------
// MTLBlitCommandEncoder.hpp
// ---------------------------------------------------------------------------

mtl_options! {
    /// `MTL::BlitOption`.
    MTLBlitOption : u64 {
        BlitOptionNone = 0,
        BlitOptionDepthFromDepthStencil = 1,
        BlitOptionStencilFromDepthStencil = 1 << 1,
        BlitOptionRowLinearPVRTC = 1 << 2,
    }
}

// ---------------------------------------------------------------------------
// MTLCaptureManager.hpp
// ---------------------------------------------------------------------------

/// `MTL::CaptureDestination` (`NS::Integer`-backed).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCaptureDestination {
    CaptureDestinationDeveloperTools = 1,
    CaptureDestinationGPUTraceDocument = 2,
}

/// `MTL::CaptureError`.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCaptureError {
    CaptureErrorNotSupported = 1,
    CaptureErrorAlreadyCapturing = 2,
    CaptureErrorInvalidDescriptor = 3,
}

// ---------------------------------------------------------------------------
// MTLBinaryArchive.hpp
// ---------------------------------------------------------------------------

/// `MTL::BinaryArchiveError`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLBinaryArchiveError {
    BinaryArchiveErrorNone = 0,
    BinaryArchiveErrorInvalidFile = 1,
    BinaryArchiveErrorUnexpectedElement = 2,
    BinaryArchiveErrorCompilationFailure = 3,
    BinaryArchiveErrorInternalError = 4,
}

// ---------------------------------------------------------------------------
// MTLRenderCommandEncoder.hpp
// ---------------------------------------------------------------------------

/// `MTL::PrimitiveType`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLPrimitiveType {
    PrimitiveTypePoint = 0,
    PrimitiveTypeLine = 1,
    PrimitiveTypeLineStrip = 2,
    PrimitiveTypeTriangle = 3,
    PrimitiveTypeTriangleStrip = 4,
}

/// `MTL::VisibilityResultMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLVisibilityResultMode {
    VisibilityResultModeDisabled = 0,
    VisibilityResultModeBoolean = 1,
    VisibilityResultModeCounting = 2,
}

/// `MTL::CullMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLCullMode {
    CullModeNone = 0,
    CullModeFront = 1,
    CullModeBack = 2,
}

/// `MTL::Winding`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLWinding {
    WindingClockwise = 0,
    WindingCounterClockwise = 1,
}

/// `MTL::DepthClipMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLDepthClipMode {
    DepthClipModeClip = 0,
    DepthClipModeClamp = 1,
}

/// `MTL::TriangleFillMode`.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MTLTriangleFillMode {
    TriangleFillModeFill = 0,
    TriangleFillModeLines = 1,
}

// Indirect draw/dispatch argument structs (MTLRenderCommandEncoder.hpp /
// MTLComputeCommandEncoder.hpp) — `#[repr(C)]` mirrors.

/// `MTL::DispatchThreadgroupsIndirectArguments`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroupsPerGrid: [u32; 3],
}

/// `MTL::DispatchThreadsIndirectArguments`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLDispatchThreadsIndirectArguments {
    pub threadsPerGrid: [u32; 3],
    pub threadsPerThreadgroup: [u32; 3],
}

/// `MTL::DrawPrimitivesIndirectArguments`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub vertexStart: u32,
    pub baseInstance: u32,
}

/// `MTL::DrawIndexedPrimitivesIndirectArguments`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub indexStart: u32,
    pub baseVertex: i32,
    pub baseInstance: u32,
}

/// `MTL::StageInRegionIndirectArguments`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLStageInRegionIndirectArguments {
    pub stageInOrigin: [u32; 3],
    pub stageInSize: [u32; 3],
}
