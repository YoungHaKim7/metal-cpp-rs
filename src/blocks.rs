//! Hand-rolled Objective-C **block literal** ABI.
//!
//! metal-cpp accepts `void (^)(...)` blocks for completion-handler APIs
//! (`MTL::CommandBuffer::addCompletedHandler`,
//! `MTL::Device::newLibrary(source, options, completionHandler)`,
//! `NS::NotificationCenter::addObserver(name, object, queue, block)`, ...).
//! This module implements the same block ABI from Rust:
//!
//! ```text
//! struct BlockLiteral {           // backed by _NSConcreteStackBlock
//!     isa, flags, reserved,
//!     invoke: fn(block, args...),
//!     descriptor: { reserved, size, copy_helper, dispose_helper },
//!     // captured environment follows the literal
//! }
//! ```
//!
//! The environment is a `Box<F>` holding the Rust closure. When Apple's
//! runtime `Block_copy`es a stack block (which every escaping API does), the
//! `copy_helper` moves ownership of the boxed closure into the heap copy and
//! the `dispose_helper` drops it when the heap block is released. If an API
//! merely *calls* the block without copying it, the boxed closure leaks —
//! intentional, and the same contract Apple's block runtime establishes for
//! stack blocks (`Block_copy` transfers ownership; without a copy there is
//! nothing to run dispose on).
//!
//! Blocks produced here are stack blocks valid for the duration of the call
//! that passes them to the Objective-C API — the same lifetime rules as C++.

use std::ffi::c_void;

use crate::objc_abi::Raw;

const BLOCK_HAS_COPY_DISPOSE: i32 = 1 << 25;
const BLOCK_HAS_DESCRIPTOR: i32 = 1 << 29;

#[repr(C)]
struct BlockLiteral {
    isa: *const c_void,
    flags: i32,
    reserved: i32,
    invoke: *const c_void,
    descriptor: *const BlockDescriptor,
}

#[repr(C)]
struct BlockDescriptor {
    reserved: usize,
    size: usize,
    copy: Option<unsafe extern "C" fn(*mut BlockLiteral, *const BlockLiteral)>,
    dispose: Option<unsafe extern "C" fn(*mut BlockLiteral)>,
}

unsafe extern "C" {
    // Blocks runtime (libSystem): concrete class for stack blocks.
    static _NSConcreteStackBlock: c_void;
    fn _Block_copy(block: *const c_void) -> *mut c_void;
    fn _Block_release(block: *const c_void);
}

/// A stack block whose single captured field is a `*mut F` (boxed closure).
/// The runtime sees `BlockLiteral` (32 bytes) followed by the capture, so
/// `invoke`/`copy`/`dispose` recover the environment by casting the block
/// pointer to this type.
#[repr(C)]
struct BlockObj<F> {
    literal: BlockLiteral,
    env: *mut F,
}

// ---------------------------------------------------------------------------
// Block0 — `void (^)(void)`
// ---------------------------------------------------------------------------

/// Rust closure typed as an Objective-C `void (^)(void)` block.
pub struct Block0<F: FnMut() + 'static> {
    obj: BlockObj<F>,
}

unsafe extern "C" fn invoke0<F: FnMut() + 'static>(block: *mut BlockLiteral) {
    let obj = block as *mut BlockObj<F>;
    let f = &mut *((*obj).env);
    f();
}

unsafe extern "C" fn copy0<F: FnMut() + 'static>(dst: *mut BlockLiteral, src: *const BlockLiteral) {
    let src = src as *const BlockObj<F>;
    let dst = dst as *mut BlockObj<F>;
    // Move ownership of the boxed closure from the stack block to the heap copy.
    (*dst).env = (*src).env;
}

unsafe extern "C" fn dispose0<F: FnMut() + 'static>(block: *mut BlockLiteral) {
    let obj = block as *mut BlockObj<F>;
    drop(Box::from_raw((*obj).env));
}

// NOTE: statics inside generic functions cannot reference the generic
// parameter, so each block creation leaks one 32-byte descriptor — fine for
// handler registrations (rare), and it keeps `copy`/`dispose` monomorphized.
fn block_descriptor0<F: FnMut() + 'static>() -> &'static BlockDescriptor {
    Box::leak(Box::new(BlockDescriptor {
        reserved: 0,
        size: std::mem::size_of::<BlockObj<F>>(),
        copy: Some(copy0::<F>),
        dispose: Some(dispose0::<F>),
    }))
}

impl<F: FnMut() + 'static> Block0<F> {
    pub fn new(f: F) -> Self {
        let env = Box::into_raw(Box::new(f));
        unsafe {
            Block0 {
                obj: BlockObj {
                    literal: BlockLiteral {
                        isa: std::ptr::addr_of!(_NSConcreteStackBlock),
                        flags: BLOCK_HAS_COPY_DISPOSE | BLOCK_HAS_DESCRIPTOR,
                        reserved: 0,
                        invoke: std::mem::transmute::<
                            unsafe extern "C" fn(*mut BlockLiteral),
                            *const c_void,
                        >(invoke0::<F>),
                        descriptor: block_descriptor0::<F>(),
                    },
                    env,
                },
            }
        }
    }

    /// Pass to an Objective-C API expecting a block.
    pub fn as_raw(&self) -> Raw {
        &self.obj as *const BlockObj<F> as Raw
    }

    /// Explicitly `Block_copy` this block; returns a heap block the caller
    /// must eventually release with [`block_release`].
    pub fn into_heap_copy(self) -> Raw {
        let raw = self.as_raw();
        std::mem::forget(self);
        unsafe { _Block_copy(raw) }
    }
}

// ---------------------------------------------------------------------------
// Block1 — `void (^)(id arg)`
// ---------------------------------------------------------------------------

/// Rust closure typed as an Objective-C `void (^)(id)` block.
pub struct Block1<F: FnMut(Raw) + 'static> {
    obj: BlockObj<F>,
}

unsafe extern "C" fn invoke1<F: FnMut(Raw) + 'static>(block: *mut BlockLiteral, a1: Raw) {
    let obj = block as *mut BlockObj<F>;
    let f = &mut *((*obj).env);
    f(a1);
}

unsafe extern "C" fn copy1<F: FnMut(Raw) + 'static>(
    dst: *mut BlockLiteral,
    src: *const BlockLiteral,
) {
    let src = src as *const BlockObj<F>;
    let dst = dst as *mut BlockObj<F>;
    (*dst).env = (*src).env;
}

unsafe extern "C" fn dispose1<F: FnMut(Raw) + 'static>(block: *mut BlockLiteral) {
    let obj = block as *mut BlockObj<F>;
    drop(Box::from_raw((*obj).env));
}

fn block_descriptor1<F: FnMut(Raw) + 'static>() -> &'static BlockDescriptor {
    Box::leak(Box::new(BlockDescriptor {
        reserved: 0,
        size: std::mem::size_of::<BlockObj<F>>(),
        copy: Some(copy1::<F>),
        dispose: Some(dispose1::<F>),
    }))
}

impl<F: FnMut(Raw) + 'static> Block1<F> {
    pub fn new(f: F) -> Self {
        let env = Box::into_raw(Box::new(f));
        unsafe {
            Block1 {
                obj: BlockObj {
                    literal: BlockLiteral {
                        isa: std::ptr::addr_of!(_NSConcreteStackBlock),
                        flags: BLOCK_HAS_COPY_DISPOSE | BLOCK_HAS_DESCRIPTOR,
                        reserved: 0,
                        invoke: std::mem::transmute::<
                            unsafe extern "C" fn(*mut BlockLiteral, Raw),
                            *const c_void,
                        >(invoke1::<F>),
                        descriptor: block_descriptor1::<F>(),
                    },
                    env,
                },
            }
        }
    }

    pub fn as_raw(&self) -> Raw {
        &self.obj as *const BlockObj<F> as Raw
    }

    pub fn into_heap_copy(self) -> Raw {
        let raw = self.as_raw();
        std::mem::forget(self);
        unsafe { _Block_copy(raw) }
    }
}

// ---------------------------------------------------------------------------
// Block2 — `void (^)(id, id)`
// ---------------------------------------------------------------------------

/// Rust closure typed as an Objective-C `void (^)(id, id)` block.
pub struct Block2<F: FnMut(Raw, Raw) + 'static> {
    obj: BlockObj<F>,
}

unsafe extern "C" fn invoke2<F: FnMut(Raw, Raw) + 'static>(
    block: *mut BlockLiteral,
    a1: Raw,
    a2: Raw,
) {
    let obj = block as *mut BlockObj<F>;
    let f = &mut *((*obj).env);
    f(a1, a2);
}

unsafe extern "C" fn copy2<F: FnMut(Raw, Raw) + 'static>(
    dst: *mut BlockLiteral,
    src: *const BlockLiteral,
) {
    let src = src as *const BlockObj<F>;
    let dst = dst as *mut BlockObj<F>;
    (*dst).env = (*src).env;
}

unsafe extern "C" fn dispose2<F: FnMut(Raw, Raw) + 'static>(block: *mut BlockLiteral) {
    let obj = block as *mut BlockObj<F>;
    drop(Box::from_raw((*obj).env));
}

fn block_descriptor2<F: FnMut(Raw, Raw) + 'static>() -> &'static BlockDescriptor {
    Box::leak(Box::new(BlockDescriptor {
        reserved: 0,
        size: std::mem::size_of::<BlockObj<F>>(),
        copy: Some(copy2::<F>),
        dispose: Some(dispose2::<F>),
    }))
}

impl<F: FnMut(Raw, Raw) + 'static> Block2<F> {
    pub fn new(f: F) -> Self {
        let env = Box::into_raw(Box::new(f));
        unsafe {
            Block2 {
                obj: BlockObj {
                    literal: BlockLiteral {
                        isa: std::ptr::addr_of!(_NSConcreteStackBlock),
                        flags: BLOCK_HAS_COPY_DISPOSE | BLOCK_HAS_DESCRIPTOR,
                        reserved: 0,
                        invoke: std::mem::transmute::<
                            unsafe extern "C" fn(*mut BlockLiteral, Raw, Raw),
                            *const c_void,
                        >(invoke2::<F>),
                        descriptor: block_descriptor2::<F>(),
                    },
                    env,
                },
            }
        }
    }

    pub fn as_raw(&self) -> Raw {
        &self.obj as *const BlockObj<F> as Raw
    }

    pub fn into_heap_copy(self) -> Raw {
        let raw = self.as_raw();
        std::mem::forget(self);
        unsafe { _Block_copy(raw) }
    }
}

/// Release a block returned by [`Block0::into_heap_copy`] /
/// [`Block1::into_heap_copy`] / [`Block2::into_heap_copy`] (`_Block_release`).
pub fn block_release(block: Raw) {
    unsafe { _Block_release(block) }
}
