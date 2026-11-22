# metal-oxide
- Rust compute kernels on Apple GPUs: compiles rustc MIR to Metal Shading Language, with typed launches from stable Rust (early alpha)
- https://github.com/rust-dd/metal-oxide

# compile install

```bash
xcodebuild -downloadComponent MetalToolchain
```

# metal파일 컴파일하기

- 예시`.metal`

```metal
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut triangle_vertex(uint vid [[vertex_id]]) {
    float2 positions[3] = {
        float2(-0.5, -0.5), float2(0.5, -0.5), float2(0.0, 0.5)
    };
    float3 colors[3] = {
        float3(1.0, 0.2, 0.2), float3(0.2, 1.0, 0.2), float3(0.2, 0.2, 1.0)
    };
    VertexOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = float4(colors[vid], 1.0);
    return out;
}

fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
```

- compile
  - There are two routes: **ahead-of-time** (`.metal` → `.metallib`, then load it) or **at runtime** (source string → library). Both end in the same `MTLLibrary` object.

## 1. Ahead-of-time with the `metal` command-line tool

Ships with Xcode's toolchain, always invoked via `xcrun`:

```bash
# Step 1: MSL source → AIR (Apple Intermediate Representation, LLVM bitcode)
xcrun -sdk macosx metal -c triangle.metal -o triangle.air

# Step 2: link .air files into a Metal library archive
xcrun -sdk macosx metallib triangle.air -o triangle.metallib
```

Common flags: `-DNAME=value` (preprocessor macros), `-std=...` (MSL language version), `-g` (debug info so GPU captures show source), `-O3`. In an Xcode project you skip the CLI entirely — every `.metal` file in the target is compiled automatically into `default.metallib` inside the app bundle.

## 2. Load the compiled library at runtime

```objc
MTLLibrary *lib = [device newLibraryWithFile:@"triangle.metallib" error:&err]; // from disk
MTLLibrary *lib = [device newDefaultLibrary];                                  // bundle's default.metallib
```

In our crate this maps to the bindings we verified above — [device.rs:569](/home/z/my-project/download/metal-cpp-rs/src/metal/device.rs) has `newLibraryWithFile` and `newLibraryWithData` (the latter takes in-memory `.metallib` bytes wrapped in a `dispatch_data_t`).

## 3. Compile at runtime from source (what `offscreen_triangle.rs` does)

```rust
let lib = device.newLibrary(SHADER_SRC, None)?; // -newLibraryWithSource:options:error:
```

Same compiler backend as `xcrun metal`, just invoked at process runtime. Under the hood, the `.metallib`/AIR is still translated to GPU-specific machine code when the library is instantiated on the device — the `metal` tool is a frontend, not a code generator for your GPU.

**Rule of thumb:** use AOT `.metallib` in a real app (faster startup, catches shader errors at build time, debuggable); use `newLibraryWithSource` when you have no Xcode build step — which is exactly the zero-dependency situation of this crate. If a GUI example later gets heavy shaders, we can ship a checked-in `.metallib` and load it with `newLibraryWithFile` instead.
