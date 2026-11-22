//! Raw Objective-C runtime ABI layer — the Rust equivalent of metal-cpp's
//! private `Object::sendMessage` / `NS::RetainPtr` machinery.
//!
//! Everything here is declared by hand against the runtime ABI, exactly the
//! way metal-cpp declares `objc_msgSend` in its `*_Private.hpp`
//! implementation branch. There are no third-party dependencies.

use std::ffi::{CStr, c_char, c_void};
use std::fmt;
use std::mem;
use std::ops::Deref;

/// `id` — untyped Objective-C object pointer.
pub type Raw = *mut c_void;
/// `NS::Integer` = `std::intptr_t` (64-bit on all supported targets).
pub type NSInteger = i64;
/// `NS::UInteger` = `std::uintptr_t` (64-bit on all supported targets).
pub type NSUInteger = u64;
/// `CGFloat` (`double` on 64-bit Apple platforms).
pub type CGFloat = f64;
/// `CFTimeInterval` (`double`).
pub type CFTimeInterval = f64;

/// `SEL` — an interned method selector.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sel(pub *const c_void);
unsafe impl Send for Sel {}
unsafe impl Sync for Sel {}

impl Sel {
    /// `sel_registerName`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn from_cstr(name: &CStr) -> Sel {
        sel_registerName(name.as_ptr())
    }

    /// The selector's registered name.
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0 as *const c_char) }
    }
}

/// `Class` — a pointer to an Objective-C class object.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Class(pub *const c_void);
unsafe impl Send for Class {}
unsafe impl Sync for Class {}

impl Class {
    /// `objc_getClass`.
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn from_cstr(name: &CStr) -> Class {
        objc_getClass(name.as_ptr())
    }

    /// A `Class` used as an `id` (for class methods the receiver *is* the class).
    pub fn ptr(self) -> Raw {
        self.0 as Raw
    }

    pub fn is_nil(self) -> bool {
        self.0.is_null()
    }
}

// ---------------------------------------------------------------------------
// Runtime entry points — libobjc
// ---------------------------------------------------------------------------

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> Class;
    fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_retain(object: Raw) -> Raw;
    pub fn objc_release(object: Raw);
    pub fn objc_autorelease(object: Raw) -> Raw;
    pub fn objc_autoreleasePoolPush() -> Raw;
    pub fn objc_autoreleasePoolPop(pool: Raw);
    pub fn objc_loadWeakRetained(location: *mut Raw) -> Raw;
    pub fn objc_storeWeak(location: *mut Raw, value: Raw) -> Raw;

    // objc_msgSend is cast to the concrete signature at every call site —
    // this is exactly how metal-cpp's `Object::sendMessage<_Ret, ...>`
    // template works.
    fn objc_msgSend();
    #[cfg(target_arch = "x86_64")]
    fn objc_msgSend_stret();
    #[cfg(target_arch = "x86_64")]
    fn objc_msgSend_fpret();
}

// ---------------------------------------------------------------------------
// Typed objc_msgSend wrappers (per-arity, like metal-cpp's sendMessage<>
// template specializations).
// ---------------------------------------------------------------------------

/// `objc_msgSend` with no arguments.
pub unsafe fn send<R>(receiver: Raw, sel: Sel) -> R {
    let f: unsafe extern "C" fn(Raw, Sel) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel)
}

/// `objc_msgSend` with one argument.
pub unsafe fn send1<A1, R>(receiver: Raw, sel: Sel, a1: A1) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1)
}

/// `objc_msgSend` with two arguments.
pub unsafe fn send2<A1, A2, R>(receiver: Raw, sel: Sel, a1: A1, a2: A2) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2)
}

/// `objc_msgSend` with three arguments.
pub unsafe fn send3<A1, A2, A3, R>(receiver: Raw, sel: Sel, a1: A1, a2: A2, a3: A3) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3)
}

/// `objc_msgSend` with four arguments.
pub unsafe fn send4<A1, A2, A3, A4, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4)
}

/// `objc_msgSend` with five arguments.
pub unsafe fn send5<A1, A2, A3, A4, A5, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4, A5) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4, a5)
}

/// `objc_msgSend` with six arguments.
pub unsafe fn send6<A1, A2, A3, A4, A5, A6, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4, A5, A6) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4, a5, a6)
}

/// `objc_msgSend` with seven arguments.
pub unsafe fn send7<A1, A2, A3, A4, A5, A6, A7, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4, A5, A6, A7) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4, a5, a6, a7)
}

/// `objc_msgSend` with eight arguments.
pub unsafe fn send8<A1, A2, A3, A4, A5, A6, A7, A8, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4, A5, A6, A7, A8) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4, a5, a6, a7, a8)
}

/// `objc_msgSend` with nine arguments.
pub unsafe fn send9<A1, A2, A3, A4, A5, A6, A7, A8, A9, R>(
    receiver: Raw,
    sel: Sel,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8,
    a9: A9,
) -> R {
    let f: unsafe extern "C" fn(Raw, Sel, A1, A2, A3, A4, A5, A6, A7, A8, A9) -> R =
        mem::transmute(objc_msgSend as unsafe extern "C" fn());
    f(receiver, sel, a1, a2, a3, a4, a5, a6, a7, a8, a9)
}

/// Struct-returning `objc_msgSend`. On `x86_64`, Apple's ABI routes returns
/// of structs larger than 16 bytes through `objc_msgSend_stret` (hidden sret
/// pointer as the first argument); on `arm64` there is no `_stret` variant.
pub unsafe fn send_stret<R>(receiver: Raw, sel: Sel) -> R {
    #[cfg(target_arch = "x86_64")]
    {
        let mut out = mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut mem::MaybeUninit<R>, Raw, Sel) =
            mem::transmute(objc_msgSend_stret as unsafe extern "C" fn());
        f(&mut out, receiver, sel);
        out.assume_init()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        send::<R>(receiver, sel)
    }
}

/// Struct-returning `objc_msgSend_stret` with one argument (x86_64 uses the
/// hidden sret pointer; arm64 uses the plain `objc_msgSend`).
pub unsafe fn send_stret1<A1, R>(receiver: Raw, sel: Sel, a1: A1) -> R {
    #[cfg(target_arch = "x86_64")]
    {
        let mut out = mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut mem::MaybeUninit<R>, Raw, Sel, A1) =
            mem::transmute(objc_msgSend_stret as unsafe extern "C" fn());
        f(&mut out, receiver, sel, a1);
        out.assume_init()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        send1::<A1, R>(receiver, sel, a1)
    }
}

/// `float`-returning `objc_msgSend`. On `x86_64`, Apple's ABI routes `float`
/// returns through `objc_msgSend_fpret`; on `arm64` there is no `_fpret`.
pub unsafe fn send_f32(receiver: Raw, sel: Sel) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "C" fn(Raw, Sel) -> f32 =
            mem::transmute(objc_msgSend_fpret as unsafe extern "C" fn());
        f(receiver, sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        send::<f32>(receiver, sel)
    }
}

/// The Objective-C `BOOL` type as it crosses the ABI: `bool` on arm64,
/// `signed char` on x86_64.
#[cfg(target_arch = "aarch64")]
pub type ObjBOOL = bool;
/// The Objective-C `BOOL` type as it crosses the ABI: `bool` on arm64,
/// `signed char` on x86_64.
#[cfg(not(target_arch = "aarch64"))]
pub type ObjBOOL = i8;

/// Convert a Rust `bool` into the ABI's `BOOL` representation.
#[inline]
pub fn bool_arg(b: bool) -> ObjBOOL {
    #[cfg(target_arch = "aarch64")]
    {
        b
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        b as ObjBOOL
    }
}

/// Call a method that returns `BOOL` and convert the result to `bool`.
pub unsafe fn send_bool(receiver: Raw, sel: Sel) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        send::<bool>(receiver, sel)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        send::<ObjBOOL>(receiver, sel) != 0
    }
}

/// Call a one-argument method that returns `BOOL`.
pub unsafe fn send1_bool<A1>(receiver: Raw, sel: Sel, a1: A1) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        send1::<A1, bool>(receiver, sel, a1)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        send1::<A1, ObjBOOL>(receiver, sel, a1) != 0
    }
}

// ---------------------------------------------------------------------------
// Static selector / class caches (the macros used across this crate).
// ---------------------------------------------------------------------------

/// `sel!("newCommandQueue")` — lazily registers and caches a selector.
macro_rules! sel {
    ($name:literal) => {{
        static SEL: std::sync::LazyLock<$crate::objc_abi::Sel> = std::sync::LazyLock::new(|| {
            let c = std::ffi::CStr::from_bytes_with_nul(concat!($name, "\0").as_bytes())
                .expect("selector literals must not contain interior NUL bytes");
            unsafe { $crate::objc_abi::Sel::from_cstr(c) }
        });
        *std::sync::LazyLock::force(&SEL)
    }};
}
pub(crate) use sel;

/// `class!("MTLDevice")` — lazily looks up and caches an Objective-C class.
/// Panics if the class is not registered (i.e. not running on Apple
/// platforms, or the owning framework was not linked/loaded).
macro_rules! class {
    ($name:literal) => {{
        static CLS: std::sync::LazyLock<$crate::objc_abi::Class> = std::sync::LazyLock::new(|| {
            let c = std::ffi::CStr::from_bytes_with_nul(concat!($name, "\0").as_bytes())
                .expect("class-name literals must not contain interior NUL bytes");
            let cls = unsafe { $crate::objc_abi::Class::from_cstr(c) };
            if cls.is_nil() {
                panic!(
                    "Objective-C class '{}' not found — this crate requires Apple platforms",
                    concat!($name)
                );
            }
            cls
        });
        *std::sync::LazyLock::force(&CLS)
    }};
}
pub(crate) use class;

/// Declares a typed wrapper `struct` over an `id` for an Objective-C class —
/// the Rust equivalent of one metal-cpp class declaration.
macro_rules! objc_class {
    ($(#[$m:meta])* $vis:vis struct $name:ident : $cls:literal;) => {
        $(#[$m])*
        #[repr(transparent)]
        $vis struct $name($crate::objc_abi::Raw);

        impl $crate::objc_abi::Obj for $name {
            fn from_raw(ptr: $crate::objc_abi::Raw) -> Self {
                $name(ptr)
            }
            fn as_raw(&self) -> $crate::objc_abi::Raw {
                self.0
            }
            unsafe fn class_ptr() -> $crate::objc_abi::Class {
                $crate::objc_abi::class!($cls)
            }
        }

        impl $name {
            /// The nil object (metal-cpp `nullptr`).
            pub fn nil() -> Self {
                $name(std::ptr::null_mut())
            }
            pub fn is_nil(&self) -> bool {
                self.0.is_null()
            }
            /// `+[Cls alloc]` — owning (+1), *uninitialized* object.
            pub unsafe fn alloc() -> $crate::objc_abi::Strong<Self> {
                $crate::objc_abi::alloc::<Self>()
            }
            /// `[[Cls alloc] init]` — owning (+1), initialized object.
            pub unsafe fn new() -> $crate::objc_abi::Strong<Self> {
                $crate::objc_abi::new_obj::<Self>()
            }
            /// The Objective-C `Class` object.
            pub fn class() -> $crate::objc_abi::Class {
                $crate::objc_abi::class!($cls)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:p})", stringify!($name), self.0)
            }
        }
    };
}
pub(crate) use objc_class;

// ---------------------------------------------------------------------------
// Object model: Obj trait, Strong / Ref (metal-cpp RetainPtr / TransferPtr).
// ---------------------------------------------------------------------------

/// Every wrapper type implements this; it is the Rust stand-in for the common
/// `NS::Object` base of metal-cpp.
pub trait Obj: Sized {
    /// Wrap a raw `id` without any retain/release.
    fn from_raw(ptr: Raw) -> Self;
    /// The underlying `id`.
    fn as_raw(&self) -> Raw;

    /// The class object for this type (used by `alloc`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn class_ptr() -> Class;

    fn is_nil(&self) -> bool {
        self.as_raw().is_null()
    }

    /// `-isKindOfClass:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn isKindOfClass(&self, cls: Class) -> bool {
        send1_bool(self.as_raw(), sel!("isKindOfClass:"), cls)
    }

    /// `-respondsToSelector:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn respondsToSelector(&self, sel: Sel) -> bool {
        send1_bool(self.as_raw(), sel!("respondsToSelector:"), sel)
    }

    /// `-isEqual:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn isEqual(&self, other: Raw) -> bool {
        send1_bool(self.as_raw(), sel!("isEqual:"), other)
    }

    /// `-hash`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn hash(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("hash"))
    }

    /// `-description`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn description(&self) -> Ref<crate::foundation::NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("description")))
    }

    /// `-retain`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn retain(&self) {
        objc_retain(self.as_raw());
    }

    /// `-release`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn release(&self) {
        objc_release(self.as_raw());
    }

    /// `-autorelease`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    unsafe fn autorelease(&self) {
        objc_autorelease(self.as_raw());
    }
}

/// Owning reference (`+1`). The Rust equivalent of metal-cpp's
/// `NS::SharedPtr<T>` obtained via `RetainPtr`/`TransferPtr`: releases the
/// object on drop. May hold nil.
pub struct Strong<T: Obj>(T);

impl<T: Obj> Strong<T> {
    /// Take ownership of an existing +1 reference (`NS::TransferPtr`).
    pub unsafe fn from_transfer(ptr: Raw) -> Self {
        Strong(T::from_raw(ptr))
    }

    /// Retain a +0 reference (`NS::RetainPtr`).
    pub unsafe fn from_retain(ptr: Raw) -> Self {
        Strong(T::from_raw(objc_retain(ptr)))
    }

    /// Nil-owned reference.
    pub fn nil() -> Self {
        Strong(T::from_raw(std::ptr::null_mut()))
    }

    pub fn ptr(&self) -> Raw {
        self.0.as_raw()
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    /// Non-owning view (`&T` in metal-cpp).
    pub fn as_ref(&self) -> Ref<T> {
        Ref(T::from_raw(self.0.as_raw()))
    }

    /// Relinquish ownership (`+0`) — the caller is responsible for the
    /// reference count (`SharedPtr::detach` in metal-cpp).
    pub unsafe fn into_raw(self) -> Raw {
        let ptr = self.0.as_raw();
        mem::forget(self);
        ptr
    }

    /// Hand the object to the current autorelease pool (`-autorelease`),
    /// converting into a non-owning view.
    pub unsafe fn autorelease(self) -> Ref<T> {
        let ptr = self.0.as_raw();
        objc_autorelease(ptr);
        mem::forget(self);
        Ref(T::from_raw(ptr))
    }
}

impl<T: Obj> Drop for Strong<T> {
    fn drop(&mut self) {
        if !self.0.is_nil() {
            unsafe { objc_release(self.0.as_raw()) }
        }
    }
}

impl<T: Obj> Deref for Strong<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Obj + fmt::Debug> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

/// Non-owning reference — the Rust stand-in for the raw `T*` that metal-cpp
/// methods return. Valid as long as someone else keeps the object alive
/// (typically the enclosing `NS::AutoreleasePool` / [`autoreleasepool`]).
pub struct Ref<T: Obj>(T);

impl<T: Obj> Ref<T> {
    /// Wrap a raw `id` as a non-owning view (may be nil, like metal-cpp).
    pub unsafe fn from_raw(ptr: Raw) -> Self {
        Ref(T::from_raw(ptr))
    }

    pub fn ptr(&self) -> Raw {
        self.0.as_raw()
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    /// Turn the borrowed view into an owning `+1` reference.
    pub unsafe fn retain(&self) -> Strong<T> {
        Strong::from_retain(self.0.as_raw())
    }
}

impl<T: Obj> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Ref(T::from_raw(self.0.as_raw()))
    }
}

impl<T: Obj> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Obj + fmt::Debug> fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

// ---------------------------------------------------------------------------
// alloc / init helpers
// ---------------------------------------------------------------------------

/// `+[Cls alloc]` as a raw `id` (+1, uninitialized).
pub unsafe fn alloc_raw(cls: Class) -> Raw {
    send::<Raw>(cls.ptr(), sel!("alloc"))
}

/// `-init` as a raw `id` (consumes the alloc's +1, returns the same object).
pub unsafe fn init_raw(obj: Raw) -> Raw {
    send::<Raw>(obj, sel!("init"))
}

/// Typed `+[Cls alloc]`.
pub unsafe fn alloc<T: Obj>() -> Strong<T> {
    Strong::from_transfer(alloc_raw(T::class_ptr()))
}

/// Typed `[[Cls alloc] init]`.
pub unsafe fn new_obj<T: Obj>() -> Strong<T> {
    Strong::from_transfer(init_raw(alloc_raw(T::class_ptr())))
}

// ---------------------------------------------------------------------------
// Autorelease pool
// ---------------------------------------------------------------------------

/// Runs `f` inside a new autorelease pool using the runtime ABI
/// (`objc_autoreleasePoolPush` / `objc_autoreleasePoolPop`) — the same
/// mechanism `@autoreleasepool` uses in Objective-C. metal-cpp exposes the
/// same concept through `NS::AutoreleasePool` (see
/// [`crate::foundation::NSAutoreleasePool`]).
///
/// Note: if `f` panics, the pool is not popped.
pub fn autoreleasepool<R, F: FnOnce() -> R>(f: F) -> R {
    let pool = unsafe { objc_autoreleasePoolPush() };
    let result = f();
    unsafe { objc_autoreleasePoolPop(pool) };
    result
}
