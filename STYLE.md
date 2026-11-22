# Rust House Style — metal-cpp-rs

Applies to every Rust file in this crate (`src/`, `examples/`, `tests/`).
This is the common house style adapted to this project: an **ABI-faithful,
zero-dependency, unsafe-by-nature Objective-C binding** of Apple's
metal-cpp (macOS 27 / iOS 27 SDK era, Metal 4), implemented concretely
against the Objective-C ABI — the same technique metal-cpp itself uses to
bridge C++ to the runtime. `unsafe` is not an exception here — it is the
API surface — so the generic rules are adjusted below.

**macOS-only.** `aarch64-apple-darwin` is the target of record and the
only target that needs checking (`cargo check --all-targets
--target aarch64-apple-darwin`). The `windows` source group below is
therefore always empty here, and the `#[cfg_attr(target_vendor = "apple", …)]`
gates exist only so `cargo check` runs on non-Mac hosts — not because
other platforms are supported.

**Minimize file movement.** Never split, merge, rename, or relocate files;
when a file is sent for reorganization, reorder it *in place* into the shape
below (sort items within the file), keep the grouped `use` block, and apply
the spacing rules. Reorganization is mechanical; line-level formatting
(indentation, wrapping) is settled afterwards by `cargo fmt`.

---

## 1. File header (before the `use` block)

In this exact order:

1. `//!` module-level doc comments — very first, nothing above them
2. `#![...]` inner attributes (`#![allow(...)]`, `#![cfg_attr(...)]`, etc.)
3. `extern crate` (not used in this crate; if present it goes here, above `use`)

```rust
//! `MTL::Device` + `MTL::Architecture` (`Metal/MTLDevice.hpp`).

#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use std::{ … };
```

Crate-wide lint allowances live **once** in `src/lib.rs`
(`non_snake_case`, `non_camel_case_types`, `non_upper_case_globals`,
`unsafe_op_in_unsafe_fn`, the clippy set). Do not repeat them per file;
add a file-level `#![allow(...)]` only when that file genuinely needs an
extra lint exception (e.g. `src/metal/enums.rs`).

## 2. Top-of-file `use` block

Four source groups, in this order, separated by a blank line:

1. `std` — standard library (`std`, `core`, `alloc` count as std here)
2. `external` — third-party crates (in `examples/` and `tests/` this is
   `metal_cpp_rs`; the library itself is zero-dependency, so this group is
   normally empty in `src/`)
3. `windows` — never used in this macOS-only crate; kept in the scheme for completeness
4. `crate` — own crate: `crate::`, `super::`, `self::`, and bare relative
   paths; order the statements `crate` → `self` → `super` within the group

```rust
use std::ffi::c_void;

use crate::{
    foundation::NSString,
    metal::enums::MTLPixelFormat,
    objc_abi::{objc_class, sel, send1, NSUInteger, Obj, Raw, Ref, Strong},
};
```

`pub use` follows the same source grouping — within a group, `use` comes
before `pub use` (private plumbing first, public re-exports second).

### 2.1 Brace grouping — one `use` statement per crate root

**Within one root, collapse all imports sharing a common prefix of two or
more segments into a single braced `use` statement.**

```rust
// Ungrouped — one line per import
use crate::foundation::NSString;
use crate::metal::enums::MTLPixelFormat;
use crate::objc_abi::{objc_class, sel};
use crate::objc_abi::{send1, Raw, Strong};

// Preferred — single statement per root, merged and deduplicated
use crate::{
    foundation::NSString,
    metal::enums::MTLPixelFormat,
    objc_abi::{objc_class, sel, send1, Raw, Strong},
};
```

Two project-specific carve-outs:

- **Relative single-segment re-exports stay one statement per module.**
  The `pub use submodule::*;` glob re-exports in `src/metal/mod.rs` (and
  the `pub use crate::…::*;` lines in `prelude`) are kept as individual
  statements, sorted alphabetically — do **not** fold them into a
  `pub use self::{…}` tree.
- **Macro re-export lines are glued to their definition, not to the `use`
  block.** `pub(crate) use sel;` / `pub(crate) use class;` /
  `pub(crate) use objc_class;` in `src/objc_abi.rs` must stay immediately
  below their `macro_rules!` definition (macro_rules scoping is textual).
  They are not moved to the top-of-file `use` block.

When merging statements, union the leaves; a leaf that already exists is
silently deduplicated (never emitted twice — that would be E0252).

### 2.2 Within a brace group, ordering

1. `self` first, then glob `*`, then everything else.
2. The rest alphabetically, segment by segment, case-insensitive
   (`de` before `Deserialize`, matching `rustfmt`).
3. When a segment has both a leaf item and sub-paths, merge them with
   `self`.

```rust
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};
```

### 2.3 Formatting

A braced group renders inline (`use a::{b, c};`) when the whole statement
fits in 100 columns; otherwise it expands one leaf per line, nested groups
indented by 4, short nested groups allowed to stay inline:

```rust
use crate::{
    foundation::{NSArray, NSError, NSString},
    metal::enums::{MTLCommandBufferErrorOption, MTLCommandBufferStatus, MTLDispatchType},
    objc_abi::{
        objc_class, sel, send, send1, send2, send3, send4, send5, send6, send7, send8, send9,
        NSUInteger, Obj, Raw, Ref, Strong,
    },
};
```

Line-level formatting here matches `cargo fmt` defaults — after
reorganizing a file, run `cargo fmt` and keep its output.

## 3. Module-level item order

| # | Category | Notes for this crate |
|---|----------|----------------------|
| 1 | `//!` module docs | file header, nothing above |
| 2 | `#![...]` inner attributes | crate-wide allows belong in `lib.rs` only |
| 3 | `extern crate` | unused |
| 4 | `use` / `pub use` | per §2 |
| 5 | `mod` / `pub mod` | module tree |
| 6 | `extern "…" { }` blocks | FFI imports: `objc_msgSend` family, retain/release, `_NSConcreteStackBlock`, framework link shims with `#[cfg_attr(target_vendor = "apple", link(name = "…", kind = "framework"))]` |
| 7 | `const` / `static` | no blank line between them — see §4 |
| 8 | `type` aliases | `Raw`, `NSInteger`, `NSUInteger`, `CGFloat`, `ObjBOOL`, … |
| 9 | `macro_rules!` | `sel!`, `class!`, `objc_class!`, `mtl_options!`, `encoder_base_methods!` — must textually precede every invocation; re-export lines glued per §2.1 |
| 10 | `trait` | `Obj` |
| 11 | `struct` / `enum` / `union` | includes **type-declaring macro invocations** (`objc_class! { pub struct MTLDevice … }`, `mtl_options! { … }`), keyed by the declared type name — see §7 |
| 12 | `impl` | includes **impl-stamping macro invocations** (`encoder_base_methods!(MTLRenderCommandEncoder);`), keyed by the self type; inherent impls before trait impls of the same type |
| 13 | `fn` | safe fns: `fn`, `pub fn`, `async fn`, `const fn` |
| 14 | `unsafe fn` | the msgSend wrapper surface — this is the bulk of the crate |
| 15 | `unsafe extern "…" fn` | block-ABI thunks (`invoke0/1/2`, `copy0/1/2`, `dispose0/1/2`) |
| 16 | `fn main` | binary entry files only (`examples/*.rs`) |
| 17 | `#[cfg(test)] mod tests` | `tests/apple_tests.rs` keeps `#![cfg(target_vendor = "apple")]` in the header instead; only in-file test mods go here |

**Mirror-file exception.** `src/metal/enums.rs` is a verbatim mirror of the
metal-cpp headers: its discriminant values *are* the ABI, and its type items
are grouped by upstream header with section banners. In that one file the
type bucket (#11) keeps **upstream declaration order** (banners stay where
they are); every other bucket in that file (use block, impls) follows the
normal rules. Everywhere else, type items sort by name.

## 4. Spacing rules

- **One blank line** between top-level items of **different categories**.
- **One blank line** between adjacent items of the **same category**
  (e.g. two `impl` blocks, two `unsafe fn`s).
- **Exception:** `const` and `static` are placed **without** a blank line
  between them — they form a single "constants" block.
- **No blank line** within a single item (between its attributes, signature,
  and body).

## 5. Within-category ordering

### 5.1 Visibility (within a single category)

Order from widest to narrowest:

```
pub  →  pub(crate)  →  pub(super)  →  pub(self) / private
```

### 5.2 Attributes (on a single item)

Order outer attributes top-to-bottom; the same order applies to inner
`#![...]` attributes in the header:

1. `#[cfg(...)]` / `#[cfg_attr(...)]`
2. `#[allow(...)]` / `#[warn(...)]` / `#[deny(...)]` / `#[forbid(...)]`
3. `#[derive(...)]`
4. other attributes (`#[inline]`, `#[repr(...)]`, doc comments `///`, etc.)

```rust
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/// A selector handle (`SEL`).
pub struct Sel(pub *const core::ffi::c_void);
```

(When a doc comment is present it stays directly above the item it
documents, immediately before the signature, per rustfmt convention.)

## 6. Unsafe & ABI conventions (project-specific)

These rules are the reason this style exists; they are mandatory.

### 6.1 `unsafe` surface

- The public API mirrors metal-cpp's C++ headers: **every method that sends
  a message is `pub unsafe fn`**, with a `///` doc comment carrying the
  exact Objective-C selector (`/// `-setLabel:``) and a `# Safety` section
  ("Calls into the Objective-C runtime." plus any extra contract).
- `src/lib.rs` declares `#![allow(unsafe_op_in_unsafe_fn)]` crate-wide.
  Do **not** wrap msgSend calls inside `unsafe fn` bodies in nested
  `unsafe { }` blocks and do **not** sprinkle per-item allows.
- Any `unsafe { }` block inside *safe* code (`autoreleasepool`, `Drop`,
  the block thunks) carries a short SAFETY rationale, in a `// SAFETY:`
  comment or in prose where the block is self-evident.

### 6.2 Selectors and class names are ABI

- Selector strings must be the **verbatim** strings resolved by metal-cpp's
  `_MTL_PRIVATE_SEL` call sites (`MTLHeaderBridge.hpp`, `NSPrivate.hpp`,
  `CAPrivate.hpp`). Never reformat, rename, or "clean up" a selector.
- Selectors are written `sel!("setLabel:")`, classes `class!("MTLDevice")`
  — both lazily cache through the macros in `objc_abi.rs`.
- Enum discriminants, struct layouts and `BOOL` widths are likewise
  verbatim; the arm64/x86_64 `BOOL` difference is expressed through
  `ObjBOOL` + the `send*_bool` helpers, not by editing signatures ad hoc.

### 6.3 FFI declarations

- Runtime entry points live in `unsafe extern "C" { }` blocks in the module
  that uses them (bucket #6), with the raw ABI in `src/objc_abi.rs` and
  framework shims in the framework's module (`metal/mod.rs`, `foundation.rs`,
  `quartz_core.rs`).
- Framework links use
  `#[cfg_attr(target_vendor = "apple", link(name = "…", kind = "framework"))]`
  on the extern block, and each linked framework must have at least one
  referenced exported symbol (the force-link pattern: `NSStringFromClass`,
  `MTLCreateSystemDefaultDevice`, the `CAMetalLayer` class symbol) so the
  dylib is actually loaded at process start.

### 6.4 Ownership mirrors metal-cpp

- `Strong<T>` = `NS::RetainPtr`/`TransferPtr` (+1, releases on drop);
  `Ref<T>` = borrowed view of the raw pointer the C++ API would return.
  Constructors returning objects return `Strong`; property getters return
  `Ref`; the `new*` / `copy*` method-name prefixes of metal-cpp are kept.
- Block-ABI thunks are `unsafe extern "C" fn` and monomorphized per arity
  (`invoke0/1/2`, `copy0/1/2`, `dispose0/1/2`) — one `Block0/1/2` type per
  arity, matching the block literal layout exactly.

### 6.5 Naming

- Types, methods and constants keep the metal-cpp spelling
  (`newCommandQueue`, `gpuResourceID`, `MTLPixelFormat::PixelFormatBGRA8Unorm`,
  `MTLSize::Make`). This is why `non_snake_case` / `non_upper_case_globals`
  are allowed crate-wide. Renaming anything here breaks the 1:1 mapping.

## 7. Reorganization procedure

When reordering an existing file **in place** (no file moves):

1. Collect every top-level item, preserving its body **verbatim** (doc
   comments, attributes, and body bytes never change).
2. Bucket each item by category from the table in §3. Macro invocations
   bucket as: type-declaring macros (`objc_class!`, `mtl_options!`) → #11;
   impl-stamping macros (`encoder_base_methods!`) → #12. The sort key for a
   macro invocation is the type name it declares (the first
   `struct`/`enum`/`union` ident inside it, or the first ident argument for
   impl stamps), not the macro name.
3. Within each bucket, sort by §5 (visibility, then key; inherent impls
   before trait impls of the same self type) — except the mirror file
   `src/metal/enums.rs`, whose type bucket keeps declaration order.
4. Reattach attributes per §5.2; banner comments (`// --- … ---` and bare
   `//` section notes) attach to the item they precede and travel with it —
   after the mechanical pass, review banner placement and re-anchor any
   banner that no longer describes what follows it.
5. Emit buckets in order, applying §4 spacing.
6. Rebuild the `use` block per §2: four source groups, blank line between
   groups, one braced statement per root (≥2-segment rule, leaf
   deduplication), `use` before `pub use`, paths ordered per §2.2. The
   exception applies recursively: inside a nested module whose body is only
   `use` statements (e.g. `pub mod prelude` in `lib.rs`), that body's use
   block is rebuilt the same way.
7. Do not rename, merge, or rewrite any non-`use` item — only reorder and
   re-space. (`use` statements are the sole exception: they are merged and
   reformatted per §2. Macro re-export lines stay glued to their
   `macro_rules!` definitions.)
8. Verify: `cargo check --all-targets --target aarch64-apple-darwin` must
   stay at **0 errors / 0 warnings** on rustc 1.100.0, and the multiset of
   item bodies (comments stripped) must be identical before and after.

## 8. Canonical skeleton

```rust
//! `MTL::CommandQueue` + `MTL::CommandQueueDescriptor`
//! (`Metal/MTLCommandQueue.hpp`).

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{
    foundation::NSString,
    metal::command_buffer::{MTLCommandBuffer, MTLCommandBufferDescriptor},
    metal::device::MTLDevice,
    objc_abi::{objc_class, sel, send, send1, NSUInteger, Obj, Raw, Ref, Strong},
};

#[cfg_attr(target_vendor = "apple", link(name = "Metal", kind = "framework"))]
unsafe extern "C" {
    #[link_name = "MTLCreateSystemDefaultDevice"]
    fn MTLCreateSystemDefaultDevice_c() -> Raw;
}

const BLOCK_HAS_COPY_DISPOSE: i32 = 1 << 25;
const BLOCK_HAS_DESCRIPTOR: i32 = 1 << 29;

pub type Raw = *mut c_void;

macro_rules! sel {
    ($name:literal) => { … };
}
pub(crate) use sel;

pub trait Obj: Sized { … }

objc_class!(
    /// `MTL::CommandQueue` (`Metal/MTLCommandQueue.hpp`).
    pub struct MTLCommandQueue : "MTLCommandQueue";
);

impl MTLCommandQueue {
    /// `-label`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn label(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("label")))
    }
}

pub unsafe fn MTLCreateSystemDefaultDevice() -> Strong<MTLDevice> { … }

unsafe extern "C" fn copy0<F: FnMut() + 'static>(dst: *mut BlockLiteral, src: *const BlockLiteral) { … }

fn main() { … }

#[cfg(test)]
mod tests { … }
```
