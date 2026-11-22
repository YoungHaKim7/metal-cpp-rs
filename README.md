<p align="center">
  <img alt="apple" width=30px src="https://user-images.githubusercontent.com/67513038/210920737-91c0c09a-e8b1-4343-92d5-17db686c2373.png" />
  <img alt="rust" width=30px src="https://github.com/user-attachments/assets/3489669b-63c0-439e-b507-9b2bfb3fdd5e" />
  <a href="https://www.rust-lang.org/" rel="nofollow noopener noreferrer">
    <img src="https://img.shields.io/badge/Rust-1.100+-orange.svg" alt="Rust">
  </a>
</p>

# metal-cpp-rs

**ABI-faithful Rust conversion of Apple's
[`metal-cpp`](https://github.com/apple/metal-cpp)** (macOS 27 / iOS 27 SDK
era, Metal 4), implemented *concretely against the Objective-C ABI* — the
same technique metal-cpp itself uses to bridge C++ to the runtime.

Crate version `1.100.0`, built and checked with **rustc 1.100.0 stable**
(edition 2024, `rust-version = "1.100.0"`) — matching the requested
toolchain version exactly. **Zero third-party dependencies.**

```
$ rustc --version
rustc 1.100.0 (0ef1bc386 2026-11-09)
$ cargo check --all-targets     # works on any host
    Finished `dev` profile (0 errors, 0 warnings)
```

---

## 1. How the conversion works

metal-cpp wraps every Metal class in a C++ class whose `_MTL_INLINE` methods
call `objc_msgSend` with selector strings registered in
`Metal/MTLHeaderBridge.hpp` / `Foundation/NSPrivate.hpp` /
`QuartzCore/CAPrivate.hpp`. This crate replicates that from Rust:

1. **Raw runtime ABI** — [`src/objc_abi.rs`](src/objc_abi.rs) declares
   `objc_msgSend`, `objc_msgSend_stret`, `objc_msgSend_fpret`,
   `objc_getClass`, `sel_registerName`, `objc_retain/release/autorelease`,
   `objc_autoreleasePoolPush/Pop` by hand via `unsafe extern "C"`.
   `objc_msgSend` is cast to a concrete typed signature at each call site —
   the direct analogue of metal-cpp's `Object::sendMessage<_Ret, ...>`
   template.

2. **Exact selectors** — every wrapper method uses the selector string that
   metal-cpp's `_MTL_PRIVATE_SEL` call sites resolve to (verified against
   the 1454-entry bridge table of `main` @ tag
   `release/metal-cpp_macOS27.0.1_iOS27.0.1`). Where metal-cpp's C++ method
   name differs from the selector (e.g. `MTL::Device::newBuffer(length,
   options)` sends `newBufferWithLength:options:`, or
   `ComputePipelineDescriptor::threadGroupSizeIsMultipleOf...` with the
   capital "Group"), the *selector* is what reaches the runtime — exactly
   like upstream.

3. **Exact struct layouts** — `_MTL_PACKED` structs are `#[repr(C)]`
   mirrors: [`metal::types`](src/metal/types.rs) (`MTLSize`, `MTLRegion`,
   `MTLViewport`, `MTLClearColor`, `MTLResourceID { _impl: uint64_t }`,
   ...), [`foundation::NSRange`], `NSOperatingSystemVersion`,
   [`quartz_core::CGSize`].

4. **Exact enum discriminants** — [`metal::enums`](src/metal/enums.rs)
   contains the complete, verbatim value tables (e.g. all 152
   `MTLPixelFormat` values, `MTLGPUFamily::GPUFamilyApple11 = 1011`,
   `LanguageVersion4_0 = 262144`, the current-metal-cpp
   `HazardTrackingMode` ordering, `ColorWriteMask` with `Red = 1 << 3`).
   Duplicate-discriminant aliases (`UTF16StringEncoding`,
   `CommandBufferErrorAccessRevoked`) become associated consts, mirroring
   how the C++ headers alias enumerators.

### x86_64 / arm64 ABI details handled explicitly

| Detail | Handling |
|---|---|
| Struct returns > 16 bytes (`MTLSize`, `MTLViewport`, `MTLClearColor`, `NSOperatingSystemVersion`, ...) | `objc_msgSend_stret` with hidden sret pointer on `x86_64`; plain `objc_msgSend` on `arm64` ([`send_stret`, `send_stret1`]) |
| `float` returns (`Texture::minLOD`, sampler LOD clamps) | `objc_msgSend_fpret` on `x86_64` ([`send_f32`]) |
| `BOOL` type | `bool` on `arm64`, `signed char` on `x86_64` (`ObjBOOL`, `bool_arg`, `send_bool`) |
| 16-byte structs (`NSRange`, `CGSize`, `SizeAndAlign`, `ResourceID`) | returned in registers — plain `objc_msgSend` on both architectures |
| Blocks | hand-rolled block literals ([`src/blocks.rs`]) |

### Hand-rolled ObjC block ABI

[`src/blocks.rs`](src/blocks.rs) implements stack block literals backed by
`_NSConcreteStackBlock` with proper `copy_helper`/`dispose_helper`
(`BLOCK_HAS_COPY_DISPOSE | BLOCK_HAS_DESCRIPTOR`), enabling the completion
handler APIs: `CommandBuffer::addCompletedHandler`,
`addScheduledHandler`, `Device::newLibraryWithCompletionHandler` and
`NSNotificationCenter::addObserver` (block-based observer API). When the
runtime `Block_copy`es the block (every escaping API does), ownership of the
boxed closure moves to the heap copy and is dropped when the heap block is
released — Apple's exact block ownership contract.

### Ownership model (metal-cpp `NSSharedPtr.hpp` ↔ Rust)

| metal-cpp | Rust | Meaning |
|---|---|---|
| `NS::TransferPtr(p)` (`new*`, `copy`, C `CopyAllDevices`) | `Strong::from_transfer` / `Strong<T>` return values | owns the `+1`; releases on drop |
| `NS::RetainPtr(p)` (class factories like `stringWithCString:`, `layer`, `renderPassDescriptor`, `MTLCreateSystemDefaultDevice`) | `Strong::from_retain` | retains a `+0` autoreleased object |
| raw `T*` getters | `Ref<T>` return values | non-owning view; keep the enclosing [`autoreleasepool`] alive |
| `NS::SharedPtr::detach` | `Strong::into_raw` | relinquish without release |
| `NS::SharedPtr::reset`/RAII | `Drop for Strong` | `objc_release` |

Methods with `NS::Error**` out-params take `&mut Strong<NSError>` and fill
it with a transferred error object — the metal-cpp `error:` convention.

### Framework force-linking

A Rust binary does not load `Foundation`/`Metal`/`QuartzCore` by default, so
the crate force-links each framework: `MTLCreateSystemDefaultDevice` /
`MTLCopyAllDevices` (Metal), `NSStringFromClass` (Foundation) and the
`OBJC_CLASS_$_CAMetalLayer` symbol (QuartzCore) are referenced from
`#[link(..., kind = "framework")]` extern blocks.

---

## 2. Module map (metal-cpp header → Rust module)

| metal-cpp | Rust | Status |
|---|---|---|
| `Foundation/Object.hpp` | `foundation::NSObject` + `objc_abi::Obj` trait | ✅ |
| `Foundation/NSString.hpp` | `foundation::NSString` | ✅ |
| `Foundation/NSArray.hpp` / `NSDictionary.hpp` / `NSNumber.hpp` | `foundation::NSArray<T>` / `NSDictionary<K,V>` / `NSNumber` | ✅ |
| `Foundation/NSProcessInfo.hpp` | `foundation::NSProcessInfo` (incl. `_stret` `operatingSystemVersion`) | ✅ |
| `Foundation/NSNotification.hpp` | `NSNotificationCenter` / `NSNotification` (block observers) | ✅ |
| `Foundation/NSBundle.hpp` / `NSError.hpp` / `NSURL.hpp` | `NSBundle` / `NSError` / `NSURL` | ✅ |
| `Foundation/NSAutoreleasePool.hpp` / `NSRange.hpp` | `NSAutoreleasePool` / `NSRange` + `objc_abi::autoreleasepool` (runtime push/pop) | ✅ |
| `QuartzCore/CAMetalLayer.hpp` / `CAMetalDrawable.hpp` | `quartz_core::CAMetalLayer` / `CAMetalDrawable` | ✅ |
| `Metal/MTLTypes.hpp` | `metal::types` + `MTLContentionRelief` | ✅ |
| `Metal/MTLPixelFormat.hpp` | `metal::enums::MTLPixelFormat` (152/152 values) | ✅ |
| `Metal/MTLDataType.hpp` | `MTLDataType` (97/97 values) | ✅ |
| `Metal/MTLResource.hpp` | `metal::resources::MTLResource` + storage/cache/hazard/options enums | ✅ |
| `Metal/MTLBuffer.hpp` | `MTLBuffer` (contents/gpuAddress/didModifyRange/newTexture/...) | ✅ |
| `Metal/MTLTexture.hpp` | `MTLTexture` + `TextureDescriptor` + `TextureViewDescriptor`(partial) | ✅ / ◐ |
| `Metal/MTLSampler.hpp` | `SamplerDescriptor` + `SamplerState` | ✅ |
| `Metal/MTLHeap.hpp` | `MTLHeap` + `HeapDescriptor` | ✅ |
| `Metal/MTLDevice.hpp` | `metal::device::MTLDevice` (~60 methods) + `Architecture` + C entry points | ✅ (core subset) |
| `Metal/MTLCommandQueue.hpp` | `MTLCommandQueue` + descriptor | ✅ |
| `Metal/MTLCommandBuffer.hpp` | `MTLCommandBuffer` + descriptor (blocks for handlers) | ✅ |
| `Metal/MTLCommandEncoder.hpp` | `MTLCommandEncoder` + inherited surface stamped into subclasses | ✅ |
| `Metal/MTLRenderCommandEncoder.hpp` | `MTLRenderCommandEncoder` (~55 methods incl. all draw variants, fences, barriers, residency) | ✅ (core subset) |
| `Metal/MTLComputeCommandEncoder.hpp` | `MTLComputeCommandEncoder` (~30 methods) | ✅ |
| `Metal/MTLBlitCommandEncoder.hpp` | `MTLBlitCommandEncoder` (~18 methods) | ✅ |
| `Metal/MTLParallelRenderCommandEncoder.hpp` | `MTLParallelRenderCommandEncoder` | ✅ |
| `Metal/MTLRenderPass.hpp` | `metal::renderpass` (descriptor + all attachment descriptors) | ✅ |
| `Metal/MTLRenderPipeline.hpp` | `metal::pipeline` (descriptor/color attachments/state) | ✅ (core subset) |
| `Metal/MTLComputePipeline.hpp` | `MTLComputePipelineDescriptor` / `MTLComputePipelineState` | ✅ |
| `Metal/MTLDepthStencil.hpp` | `DepthStencilDescriptor` / `StencilDescriptor` / `DepthStencilState` | ✅ |
| `Metal/MTLVertexDescriptor.hpp` | `VertexDescriptor` family | ✅ |
| `Metal/MTLLibrary.hpp` | `metal::shader` (`Library`, `Function`, `VertexAttribute`, `CompileOptions`) | ✅ |
| `Metal/MTLFunctionDescriptor.hpp` / `MTLFunctionConstantValues.hpp` | `FunctionDescriptor` / `FunctionConstantValues` | ✅ |
| `Metal/MTLArgument.hpp` / `MTLArgumentEncoder.hpp` | `metal::argument` (`ArgumentDescriptor`, `ArgumentEncoder`) | ✅ |
| `Metal/MTLEvent.hpp` / `MTLFence.hpp` | `metal::sync` (`Event`, `SharedEvent`, `Fence`) | ✅ |
| `Metal/MTLCaptureManager.hpp` / `MTLCaptureScope.hpp` | `metal::capture` | ✅ |
| `Metal/MTLBinaryArchive.hpp` | `BinaryArchive(Descriptor)` — only verified core surface | ◐ |
| `Metal/MTLAccelerationStructure*.hpp`, `MTLIndirectCommandBuffer*.hpp`, `MTLRasterizationRateMap.hpp`, `MTLCounter*.hpp`, `MTL4*.hpp`, `MetalFX/` | — not mapped (out of "core scope") | ❌ |

*Note:* metal-cpp has **no CoreFoundation module** (it toll-free-bridges via
the system `<CoreFoundation/CoreFoundation.h>`); this conversion mirrors
that by not adding one.

---

## 3. Usage

```rust
use metal_cpp_rs::objc_abi::autoreleasepool;
use metal_cpp_rs::prelude::*;

autoreleasepool(|| unsafe {
    // MTL::CreateSystemDefaultDevice()
    let device = MTLCreateSystemDefaultDevice();
    println!("{}", device.name().to_string());

    // MTL::Device::newBuffer(length, options) → "newBufferWithLength:options:"
    let buffer = device.newBuffer(256, MTLResourceOptions::ResourceStorageModeShared);

    // MTL::CommandQueue::commandBuffer() etc.
    let queue = device.newCommandQueue();
    let cb = queue.commandBuffer();

    // Block API: "addCompletedHandler:"
    cb.addCompletedHandler(&Block1::new(|_buffer| {
        println!("completed!");
    }));

    cb.commit();
    cb.waitUntilCompleted();
});
```

Every method is `unsafe` (it dispatches through `objc_msgSend`); the crate
is a faithful binding layer, not a safe wrapper — same philosophy as
metal-cpp, where the caller drives lifetimes and pools.

## 4. Building

```sh
# anywhere: type-checks everything (library, examples, tests)
cargo check --all-targets

# on a Mac (Apple silicon):
cargo test  --target aarch64-apple-darwin   # integration tests (links libobjc)
cargo run   --example device_info           # prints every GPU's capabilities
cargo run   --example offscreen_triangle    # renders + writes triangle.ppm

# cross-compiling from Linux (needs the Apple target + linker):
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin
```

## 5. 100 GUI examples

`examples/001_hello_window.rs` … `examples/100_finale.rs` — one hundred
windowed Metal demos built on the tiny GUI runtime in `examples/gui/`:
an AppKit window with a layer-hosted `CAMetalLayer`, a hand-pumped
`nextEventMatchingMask:` event loop, keyboard/mouse input, a 5x7 bitmap
text renderer, an immediate-mode 2-D vertex pipeline and per-example
custom Metal shaders (`gui::make_pipeline` / `Frame::draw_custom`).
Still zero dependencies beyond this crate — ESC or the red close button
quits any example.

```sh
cargo run --release --example 001_hello_window    # the hello-world window
cargo run --release --example 082_tetris          # the app cluster starts at 080
```

| # | Examples | What they show |
|---|----------|----------------|
| 001-020 | hello window, clear color, gradient rect, rainbow triangle, spinning quad, checkerboard, bouncing ball, starfield, plasma, mandelbrot, rings, metaballs, voronoi, tunnel, kaleidoscope, synthwave grid, circle pack, polygons, star polygons, golden spiral | window + primitives + first custom shaders |
| 021-039 | lissajous, rose curve, superformula, fourier square wave, wave interference, fractal tree, koch snowflake, sierpinski, barnsley fern, dragon IFS, phyllotaxis, superellipse, grid waves, sunburst, hex tiling, flow field, pendulum wave, double pendulum, orbit sim | curves, fractals, physics |
| 040-059 | particle fountain, fireworks, confetti, galaxy, billiards, asteroids, rain, snowfall, boids, swarm, magnets, mouse trail, paint splatter, gravity wells, flame cursor, ripples, pong, breakout, flappy, runner | particles + interactive toys and mini-games |
| 060-079 | wolfram rules, langton's ant, brian's brain, cyclic CA, wa-tor, gray-scott reaction-diffusion, forest fire, wave equation, drunkard's walk, truchet tiles, julia, burning ship, newton fractal, moire, raymarched sphere / torus / menger sponge, value noise, fbm terrain, spirograph | cellular automata + raymarching |
| 080-100 | **calculator, todo app, tetris, snake, game of life**, paint, minesweeper, 2048, analog clock, stopwatch, piano, step sequencer, calendar, text editor, sine plotter, bar chart, mandelbrot explorer, color mixer, maze solver, system info, finale | full GUI apps |

## 6. Tests

`tests/apple_tests.rs` (Apple-only, `#[cfg(target_vendor = "apple")]`):
NSString/NSArray round-trips, device/queue/command-buffer lifecycle, buffer
`contents()` round-trip, a completed-handler firing through the hand-rolled
block ABI, and a 32-byte struct return (`clearColor`) exercising the
`objc_msgSend_stret` path.

## 7. Safety caveats

* All ObjC calls are `unsafe`; wrapper methods are `unsafe fn` by design
  (`unsafe_op_in_unsafe_fn` is allowed crate-wide — this is a binding layer).
* Blocks leak their boxed closure if the API never copies the block
  (documented in `blocks.rs`); escaping APIs always copy, so handlers are
  freed by the runtime's `dispose_helper`.
* `MTLCreateSystemDefaultDevice()` is wrapped with `from_retain` (the C
  function's ownership is not explicitly documented; over-retaining is the
  safe direction). `MTLCopyAllDevices()` follows the Copy rule (`from_transfer`).
* `class!()` panics if an ObjC class is missing — i.e. you are not on Apple
  platforms or the framework did not load.
* If `autoreleasepool`'s closure panics, the pool is not popped.

## 8. Provenance

Selector strings, enum values and struct layouts were extracted from
`apple/metal-cpp` `main` (= tag `release/metal-cpp_macOS27.0.1_iOS27.0.1`,
© 2020–2025 Apple Inc.): `Metal/MTLHeaderBridge.hpp` (1454 selector
definitions), `Foundation/NSPrivate.hpp` (195), `QuartzCore/CAPrivate.hpp`
(22), and the public class headers. Upstream is Apache-2.0; see
`LICENSE.txt` in the upstream repository.
