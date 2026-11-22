//! Metal struct types — `#[repr(C)]` mirrors of the `_MTL_PACKED` C++
//! structs from `Metal/MTLTypes.hpp`, `MTLRenderPass.hpp`,
//! `MTLRenderCommandEncoder.hpp`, `MTLTexture.hpp` and `MTLDevice.hpp`.
//!
//! `NS::Range` lives in [`crate::foundation::NSRange`].

use crate::metal::enums::MTLTextureSwizzle;
use crate::objc_abi::NSUInteger;

/// `MTL::Size` — `{ UInteger width; UInteger height; UInteger depth; }`
/// (24 bytes ⇒ returned via `objc_msgSend_stret` on x86_64).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}

impl MTLSize {
    /// `MTL::Size::Make(width, height, depth)`.
    pub fn Make(width: NSUInteger, height: NSUInteger, depth: NSUInteger) -> Self {
        MTLSize { width, height, depth }
    }
}

/// `MTL::Origin` — `{ UInteger x; UInteger y; UInteger z; }`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger,
}

impl MTLOrigin {
    /// `MTL::Origin::Make(x, y, z)`.
    pub fn Make(x: NSUInteger, y: NSUInteger, z: NSUInteger) -> Self {
        MTLOrigin { x, y, z }
    }
}

/// `MTL::Region` — `{ Origin origin; Size size; }`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

impl MTLRegion {
    /// `MTL::Region::Make1D(x, width)`.
    pub fn Make1D(x: NSUInteger, width: NSUInteger) -> Self {
        MTLRegion { origin: MTLOrigin::Make(x, 0, 0), size: MTLSize::Make(width, 1, 1) }
    }

    /// `MTL::Region::Make2D(x, y, width, height)`.
    pub fn Make2D(x: NSUInteger, y: NSUInteger, width: NSUInteger, height: NSUInteger) -> Self {
        MTLRegion { origin: MTLOrigin::Make(x, y, 0), size: MTLSize::Make(width, height, 1) }
    }

    /// `MTL::Region::Make3D(x, y, z, width, height, depth)`.
    pub fn Make3D(
        x: NSUInteger,
        y: NSUInteger,
        z: NSUInteger,
        width: NSUInteger,
        height: NSUInteger,
        depth: NSUInteger,
    ) -> Self {
        MTLRegion { origin: MTLOrigin::Make(x, y, z), size: MTLSize::Make(width, height, depth) }
    }
}

/// `MTL::SamplePosition` — `{ float x; float y; }` (8 bytes).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}

impl MTLSamplePosition {
    /// `MTL::SamplePosition::Make(x, y)`.
    pub fn Make(x: f32, y: f32) -> Self {
        MTLSamplePosition { x, y }
    }
}

/// `MTL::Coordinate2D` — in current metal-cpp an alias of the float-based
/// `SamplePosition` (older releases had a `double`-based struct).
pub type MTLCoordinate2D = MTLSamplePosition;

/// `MTL::ResourceID` — verbatim from `MTLTypes.hpp`:
/// `struct ResourceID { uint64_t _impl; } _MTL_PACKED;`
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MTLResourceID {
    pub _impl: u64,
}

/// `MTL::ClearColor` (declared in `MTLRenderPass.hpp`) — four `double`s
/// (32 bytes ⇒ `objc_msgSend_stret` on x86_64).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl MTLClearColor {
    /// `MTL::ClearColor::Make(red, green, blue, alpha)`.
    pub fn Make(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        MTLClearColor { red, green, blue, alpha }
    }
}

/// `MTL::Viewport` (declared in `MTLRenderCommandEncoder.hpp`) — six `double`s
/// (48 bytes ⇒ `objc_msgSend_stret` on x86_64).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MTLViewport {
    pub originX: f64,
    pub originY: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

impl MTLViewport {
    /// `MTL::Viewport::Make(originX, originY, width, height, znear, zfar)`.
    pub fn Make(originX: f64, originY: f64, width: f64, height: f64, znear: f64, zfar: f64) -> Self {
        MTLViewport { originX, originY, width, height, znear, zfar }
    }
}

/// `MTL::ScissorRect` (declared in `MTLRenderCommandEncoder.hpp`).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MTLScissorRect {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub width: NSUInteger,
    pub height: NSUInteger,
}

impl MTLScissorRect {
    /// `MTL::ScissorRect::Make(x, y, width, height)`.
    pub fn Make(x: NSUInteger, y: NSUInteger, width: NSUInteger, height: NSUInteger) -> Self {
        MTLScissorRect { x, y, width, height }
    }
}

/// `MTL::TextureSwizzleChannels` (`MTLTexture.hpp`) — four `uint8_t` swizzles.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MTLTextureSwizzleChannels {
    pub red: MTLTextureSwizzle,
    pub green: MTLTextureSwizzle,
    pub blue: MTLTextureSwizzle,
    pub alpha: MTLTextureSwizzle,
}

impl Default for MTLTextureSwizzleChannels {
    /// `TextureSwizzleChannels::Default()` — `R, G, B, A`.
    fn default() -> Self {
        MTLTextureSwizzleChannels {
            red: MTLTextureSwizzle::TextureSwizzleRed,
            green: MTLTextureSwizzle::TextureSwizzleGreen,
            blue: MTLTextureSwizzle::TextureSwizzleBlue,
            alpha: MTLTextureSwizzle::TextureSwizzleAlpha,
        }
    }
}

impl MTLTextureSwizzleChannels {
    /// `MTL::TextureSwizzleChannels::Make(r, g, b, a)`.
    pub fn Make(red: MTLTextureSwizzle, green: MTLTextureSwizzle, blue: MTLTextureSwizzle, alpha: MTLTextureSwizzle) -> Self {
        MTLTextureSwizzleChannels { red, green, blue, alpha }
    }
}

/// `MTL::SizeAndAlign` (`MTLDevice.hpp`) — `{ UInteger size; UInteger align; }`
/// (16 bytes ⇒ returned in registers, no `_stret`).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MTLSizeAndAlign {
    pub size: NSUInteger,
    pub align: NSUInteger,
}

/// `MTL::GPUAddress` (`std::uint64_t`).
pub type GPUAddress = u64;

/// `MTL::Timestamp` (`std::uint64_t`).
pub type Timestamp = u64;
