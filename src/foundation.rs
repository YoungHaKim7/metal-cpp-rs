//! Foundation module — the Rust conversion of metal-cpp's
//! `Foundation/Foundation.hpp` (`NSObject.hpp`, `NSString.hpp`,
//! `NSArray.hpp`, `NSDictionary.hpp`, `NSNumber.hpp`, `NSProcessInfo.hpp`,
//! `NSNotification.hpp`, `NSBundle.hpp`, `NSError.hpp`, `NSURL.hpp`,
//! `NSAutoreleasePool.hpp`, `NSRange.hpp`).
//!
//! Every selector matches the verbatim strings in metal-cpp's
//! `Foundation/NSPrivate.hpp` selector table (e.g. `UTF8String`,
//! `stringWithCString:encoding:`, `addObserverForName:object:queue:usingBlock:`).

use std::{
    ffi::{CStr, CString, c_char},
    marker::PhantomData,
};

use crate::{
    blocks::Block1,
    objc_abi::{
        CFTimeInterval, NSInteger, NSUInteger, Obj, Raw, Ref, Strong, bool_arg, send, send_bool,
        send_stret, send1, send1_bool, send2, send3, send4,
    },
    objc_abi::{class, objc_class, sel},
};

// ---------------------------------------------------------------------------
// Force-link the Foundation framework so the classes below are registered in
// the Objective-C runtime (a Rust binary does not load Foundation by default).
// ---------------------------------------------------------------------------

#[cfg_attr(target_vendor = "apple", link(name = "Foundation", kind = "framework"))]
unsafe extern "C" {
    #[link_name = "NSStringFromClass"]
    fn NSStringFromClass_c(cls: crate::objc_abi::Class) -> Raw;
}

/// `NSString *NSStringFromClass(Class)` — the Foundation C function wrapper
/// (this is *not* a selector; it also force-links the Foundation framework).
///
/// # Safety
/// Calls the Foundation C function.
pub unsafe fn NSStringFromClass(cls: crate::objc_abi::Class) -> Ref<NSString> {
    Ref::from_raw(NSStringFromClass_c(cls))
}

// ---------------------------------------------------------------------------
// Structs (NSRange.hpp, NSTypes.hpp) — verbatim `_NS_PACKED` layouts.
// ---------------------------------------------------------------------------

/// `NS::Range` — `_NS_PACKED { UInteger location; UInteger length; }`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    /// `NS::Range::Make(loc, len)`.
    pub fn Make(location: NSUInteger, length: NSUInteger) -> Self {
        NSRange { location, length }
    }

    /// `Range::LocationInRange(loc)`.
    pub fn LocationInRange(&self, loc: NSUInteger) -> bool {
        loc - self.location < self.length
    }

    /// `Range::Max()`.
    pub fn Max(&self) -> NSUInteger {
        self.location + self.length
    }
}

/// `NS::OperatingSystemVersion` — `_NS_PACKED { Integer majorVersion;
/// Integer minorVersion; Integer patchVersion; }`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct NSOperatingSystemVersion {
    pub majorVersion: NSInteger,
    pub minorVersion: NSInteger,
    pub patchVersion: NSInteger,
}

/// `NS::ComparisonResult` (`NSObjCRuntime.hpp`).
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ComparisonResult {
    OrderedAscending = -1,
    OrderedSame = 0,
    OrderedDescending = 1,
}

/// `NS::StringEncoding` (`NSString.hpp`) — values verbatim.
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StringEncoding {
    ASCIIStringEncoding = 1,
    NEXTSTEPStringEncoding = 2,
    JapaneseEUCStringEncoding = 3,
    UTF8StringEncoding = 4,
    ISOLatin1StringEncoding = 5,
    SymbolStringEncoding = 6,
    NonLossyASCIIStringEncoding = 7,
    ShiftJISStringEncoding = 8,
    ISOLatin2StringEncoding = 9,
    UnicodeStringEncoding = 10,
    WindowsCP1251StringEncoding = 11,
    WindowsCP1252StringEncoding = 12,
    WindowsCP1253StringEncoding = 13,
    WindowsCP1254StringEncoding = 14,
    WindowsCP1250StringEncoding = 15,
    ISO2022JPStringEncoding = 21,
    MacOSRomanStringEncoding = 30,
    UTF16BigEndianStringEncoding = 0x90000100,
    UTF16LittleEndianStringEncoding = 0x94000100,
    UTF32StringEncoding = 0x8c000100,
    UTF32BigEndianStringEncoding = 0x98000100,
    UTF32LittleEndianStringEncoding = 0x9c000100,
}

impl StringEncoding {
    /// Alias: `UTF16StringEncoding == UnicodeStringEncoding` (duplicate
    /// discriminants are not expressible as Rust enum variants).
    pub const UTF16StringEncoding: StringEncoding = StringEncoding::UnicodeStringEncoding;
}

// ---------------------------------------------------------------------------
// NSObject
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::Object` — the root wrapper class (`Foundation/NSObject.hpp`).
    pub struct NSObject : "NSObject";
);

impl NSObject {
    /// `-debugDescription`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn debugDescription(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("debugDescription")))
    }

    /// `-retainCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn retainCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("retainCount"))
    }
}

// ---------------------------------------------------------------------------
// NSString
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::String` (`Foundation/NSString.hpp`).
    pub struct NSString : "NSString";
);

impl NSString {
    /// `+[NSString string]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn string() -> Strong<NSString> {
        Strong::from_retain(send::<Raw>(NSString::class().ptr(), sel!("string")))
    }

    /// `+[NSString stringWithString:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stringWithString(string: &NSString) -> Strong<NSString> {
        Strong::from_retain(send1::<Raw, Raw>(
            NSString::class().ptr(),
            sel!("stringWithString:"),
            string.as_raw(),
        ))
    }

    /// `+[NSString stringWithCString:encoding:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stringWithCString(cString: &CStr, encoding: StringEncoding) -> Strong<NSString> {
        Strong::from_retain(send2::<*const c_char, u64, Raw>(
            NSString::class().ptr(),
            sel!("stringWithCString:encoding:"),
            cString.as_ptr(),
            encoding as u64,
        ))
    }

    /// Convenience (safe) constructor from a Rust `&str` — UTF-8 encoded,
    /// like metal-cpp's `NS::String::string(pString, UTF8StringEncoding)`.
    pub fn from_str(s: &str) -> Strong<NSString> {
        let c = CString::new(s).expect("string must not contain interior NUL");
        unsafe { NSString::stringWithCString(&c, StringEncoding::UTF8StringEncoding) }
    }

    /// `-initWithCString:encoding:` (consumes the receiver's alloc reference).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn initWithCString(
        self_: Strong<NSString>,
        cString: &CStr,
        encoding: StringEncoding,
    ) -> Strong<NSString> {
        let raw = self_.into_raw();
        Strong::from_transfer(send2::<*const c_char, u64, Raw>(
            raw,
            sel!("initWithCString:encoding:"),
            cString.as_ptr(),
            encoding as u64,
        ))
    }

    /// `-length`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn length(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("length"))
    }

    /// `-characterAtIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn characterAtIndex(&self, index: NSUInteger) -> u16 {
        send1::<NSUInteger, u16>(self.as_raw(), sel!("characterAtIndex:"), index)
    }

    /// `-cStringUsingEncoding:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn cStringUsingEncoding(&self, encoding: StringEncoding) -> *const c_char {
        send1::<u64, *const c_char>(
            self.as_raw(),
            sel!("cStringUsingEncoding:"),
            encoding as u64,
        )
    }

    /// `-UTF8String` (borrowed for the lifetime of `&self`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn UTF8String(&self) -> &CStr {
        CStr::from_ptr(send::<*const c_char>(self.as_raw(), sel!("UTF8String")))
    }

    /// Safe convenience: lossy decode of `-UTF8String`.
    pub fn to_string(&self) -> String {
        unsafe { self.UTF8String().to_string_lossy().into_owned() }
    }

    /// `-maximumLengthOfBytesUsingEncoding:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn maximumLengthOfBytesUsingEncoding(&self, encoding: StringEncoding) -> NSUInteger {
        send1::<u64, NSUInteger>(
            self.as_raw(),
            sel!("maximumLengthOfBytesUsingEncoding:"),
            encoding as u64,
        )
    }

    /// `-lengthOfBytesUsingEncoding:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn lengthOfBytesUsingEncoding(&self, encoding: StringEncoding) -> NSUInteger {
        send1::<u64, NSUInteger>(
            self.as_raw(),
            sel!("lengthOfBytesUsingEncoding:"),
            encoding as u64,
        )
    }

    /// `-isEqualToString:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isEqualToString(&self, string: &NSString) -> bool {
        send1_bool(self.as_raw(), sel!("isEqualToString:"), string.as_raw())
    }

    /// `-rangeOfString:options:` (returns `NS::Range`, a 16-byte struct that
    /// crosses the ABI in registers — no `_stret` needed).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn rangeOfString(&self, string: &NSString, options: NSUInteger) -> NSRange {
        send2::<Raw, NSUInteger, NSRange>(
            self.as_raw(),
            sel!("rangeOfString:options:"),
            string.as_raw(),
            options,
        )
    }

    /// `-fileSystemRepresentation`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fileSystemRepresentation(&self) -> *const c_char {
        send::<*const c_char>(self.as_raw(), sel!("fileSystemRepresentation"))
    }

    /// `-stringByAppendingString:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn stringByAppendingString(&self, string: &NSString) -> Strong<NSString> {
        Strong::from_retain(send1::<Raw, Raw>(
            self.as_raw(),
            sel!("stringByAppendingString:"),
            string.as_raw(),
        ))
    }

    /// `-caseInsensitiveCompare:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn caseInsensitiveCompare(&self, string: &NSString) -> ComparisonResult {
        match send1::<Raw, NSInteger>(
            self.as_raw(),
            sel!("caseInsensitiveCompare:"),
            string.as_raw(),
        ) {
            -1 => ComparisonResult::OrderedAscending,
            0 => ComparisonResult::OrderedSame,
            _ => ComparisonResult::OrderedDescending,
        }
    }
}

// ---------------------------------------------------------------------------
// NSArray / NSDictionary — generic wrappers over the (non-generic) ObjC class.
// ---------------------------------------------------------------------------

/// `NS::Array<T>` (`Foundation/NSArray.hpp`).
#[repr(transparent)]
pub struct NSArray<T: Obj>(Raw, PhantomData<*const T>);

impl<T: Obj> Obj for NSArray<T> {
    fn from_raw(ptr: Raw) -> Self {
        NSArray(ptr, PhantomData)
    }
    fn as_raw(&self) -> Raw {
        self.0
    }
    unsafe fn class_ptr() -> crate::objc_abi::Class {
        class!("NSArray")
    }
}

impl<T: Obj> NSArray<T> {
    pub fn nil() -> Self {
        NSArray(std::ptr::null_mut(), PhantomData)
    }
    pub fn is_nil(&self) -> bool {
        self.0.is_null()
    }
    /// `+[NSArray array]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn array() -> Strong<Self> {
        Strong::from_retain(send::<Raw>(Self::class_ptr().ptr(), sel!("array")))
    }
    /// `+[NSArray arrayWithObject:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn arrayWithObject(object: &T) -> Strong<Self> {
        Strong::from_retain(send1::<Raw, Raw>(
            Self::class_ptr().ptr(),
            sel!("arrayWithObject:"),
            object.as_raw(),
        ))
    }
    /// `+[NSArray arrayWithObjects:count:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn arrayWithObjects(objects: &[&T]) -> Strong<Self> {
        let ptrs: Vec<Raw> = objects.iter().map(|o| o.as_raw()).collect();
        Strong::from_retain(send2::<*const Raw, NSUInteger, Raw>(
            Self::class_ptr().ptr(),
            sel!("arrayWithObjects:count:"),
            ptrs.as_ptr(),
            ptrs.len() as NSUInteger,
        ))
    }
    /// `-count`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn count(&self) -> NSUInteger {
        send::<NSUInteger>(self.0, sel!("count"))
    }
    /// `-objectAtIndex:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectAtIndex(&self, index: NSUInteger) -> Ref<T> {
        Ref::from_raw(send1::<NSUInteger, Raw>(
            self.0,
            sel!("objectAtIndex:"),
            index,
        ))
    }
}

/// `NS::Dictionary<K, V>` (`Foundation/NSDictionary.hpp`).
#[repr(transparent)]
pub struct NSDictionary<K: Obj, V: Obj>(Raw, PhantomData<*const (K, V)>);

impl<K: Obj, V: Obj> Obj for NSDictionary<K, V> {
    fn from_raw(ptr: Raw) -> Self {
        NSDictionary(ptr, PhantomData)
    }
    fn as_raw(&self) -> Raw {
        self.0
    }
    unsafe fn class_ptr() -> crate::objc_abi::Class {
        class!("NSDictionary")
    }
}

impl<K: Obj, V: Obj> NSDictionary<K, V> {
    pub fn nil() -> Self {
        NSDictionary(std::ptr::null_mut(), PhantomData)
    }
    pub fn is_nil(&self) -> bool {
        self.0.is_null()
    }
    /// `+[NSDictionary dictionary]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dictionary() -> Strong<Self> {
        Strong::from_retain(send::<Raw>(Self::class_ptr().ptr(), sel!("dictionary")))
    }
    /// `+[NSDictionary dictionaryWithObject:forKey:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn dictionaryWithObject(object: &V, forKey: &K) -> Strong<Self> {
        Strong::from_retain(send2::<Raw, Raw, Raw>(
            Self::class_ptr().ptr(),
            sel!("dictionaryWithObject:forKey:"),
            object.as_raw(),
            forKey.as_raw(),
        ))
    }
    /// `-count`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn count(&self) -> NSUInteger {
        send::<NSUInteger>(self.0, sel!("count"))
    }
    /// `-objectForKey:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn objectForKey(&self, key: &K) -> Ref<V> {
        Ref::from_raw(send1::<Raw, Raw>(
            self.0,
            sel!("objectForKey:"),
            key.as_raw(),
        ))
    }
}

// ---------------------------------------------------------------------------
// NSNumber
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::Number` (`Foundation/NSNumber.hpp`).
    pub struct NSNumber : "NSNumber";
);

impl NSNumber {
    /// `+[NSNumber numberWithBool:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn numberWithBool(value: bool) -> Strong<NSNumber> {
        Strong::from_retain(send1::<crate::objc_abi::ObjBOOL, Raw>(
            NSNumber::class().ptr(),
            sel!("numberWithBool:"),
            bool_arg(value),
        ))
    }
    /// `+[NSNumber numberWithUnsignedLongLong:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn numberWithUnsignedLongLong(value: u64) -> Strong<NSNumber> {
        Strong::from_retain(send1::<u64, Raw>(
            NSNumber::class().ptr(),
            sel!("numberWithUnsignedLongLong:"),
            value,
        ))
    }
    /// `+[NSNumber numberWithLongLong:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn numberWithLongLong(value: i64) -> Strong<NSNumber> {
        Strong::from_retain(send1::<i64, Raw>(
            NSNumber::class().ptr(),
            sel!("numberWithLongLong:"),
            value,
        ))
    }
    /// `+[NSNumber numberWithDouble:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn numberWithDouble(value: f64) -> Strong<NSNumber> {
        Strong::from_retain(send1::<f64, Raw>(
            NSNumber::class().ptr(),
            sel!("numberWithDouble:"),
            value,
        ))
    }
    /// `-boolValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn boolValue(&self) -> bool {
        send_bool(self.as_raw(), sel!("boolValue"))
    }
    /// `-unsignedLongLongValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn unsignedLongLongValue(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("unsignedLongLongValue"))
    }
    /// `-longLongValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn longLongValue(&self) -> i64 {
        send::<i64>(self.as_raw(), sel!("longLongValue"))
    }
    /// `-doubleValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn doubleValue(&self) -> f64 {
        send::<f64>(self.as_raw(), sel!("doubleValue"))
    }
    /// `-integerValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn integerValue(&self) -> NSInteger {
        send::<NSInteger>(self.as_raw(), sel!("integerValue"))
    }
    /// `-unsignedIntegerValue`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn unsignedIntegerValue(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("unsignedIntegerValue"))
    }
}

// ---------------------------------------------------------------------------
// NSProcessInfo
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::ProcessInfo` (`Foundation/NSProcessInfo.hpp`).
    pub struct NSProcessInfo : "NSProcessInfo";
);

impl NSProcessInfo {
    /// `+[NSProcessInfo processInfo]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn processInfo() -> Strong<NSProcessInfo> {
        Strong::from_retain(send::<Raw>(
            NSProcessInfo::class().ptr(),
            sel!("processInfo"),
        ))
    }

    /// `-operatingSystemVersion` — returns `NS::OperatingSystemVersion`
    /// (24-byte struct ⇒ `objc_msgSend_stret` on x86_64).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion {
        send_stret::<NSOperatingSystemVersion>(self.as_raw(), sel!("operatingSystemVersion"))
    }

    /// `-operatingSystemVersionString`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn operatingSystemVersionString(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(
            self.as_raw(),
            sel!("operatingSystemVersionString"),
        ))
    }

    /// `-processorCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn processorCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("processorCount"))
    }

    /// `-activeProcessorCount`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn activeProcessorCount(&self) -> NSUInteger {
        send::<NSUInteger>(self.as_raw(), sel!("activeProcessorCount"))
    }

    /// `-physicalMemory`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn physicalMemory(&self) -> u64 {
        send::<u64>(self.as_raw(), sel!("physicalMemory"))
    }

    /// `-systemUptime`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn systemUptime(&self) -> CFTimeInterval {
        send::<CFTimeInterval>(self.as_raw(), sel!("systemUptime"))
    }

    /// `-isLowPowerModeEnabled`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isLowPowerModeEnabled(&self) -> bool {
        send_bool(self.as_raw(), sel!("isLowPowerModeEnabled"))
    }

    /// `-isiOSAppOnMac`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isiOSAppOnMac(&self) -> bool {
        send_bool(self.as_raw(), sel!("isiOSAppOnMac"))
    }

    /// `-isMacCatalystApp`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isMacCatalystApp(&self) -> bool {
        send_bool(self.as_raw(), sel!("isMacCatalystApp"))
    }

    /// `-hostName`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn hostName(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("hostName")))
    }

    /// `-processName`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn processName(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("processName")))
    }

    /// `-setProcessName:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn setProcessName(&self, string: &NSString) {
        send1::<Raw, ()>(self.as_raw(), sel!("setProcessName:"), string.as_raw());
    }

    /// `-globallyUniqueString`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn globallyUniqueString(&self) -> Strong<NSString> {
        Strong::from_retain(send::<Raw>(self.as_raw(), sel!("globallyUniqueString")))
    }

    /// `-environment`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn environment(&self) -> Ref<NSDictionary<NSString, NSString>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("environment")))
    }
}

// ---------------------------------------------------------------------------
// NSNotificationCenter / NSNotification
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::NotificationCenter` (`Foundation/NSNotification.hpp`).
    pub struct NSNotificationCenter : "NSNotificationCenter";
);

impl NSNotificationCenter {
    /// `+[NSNotificationCenter defaultCenter]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn defaultCenter() -> Strong<NSNotificationCenter> {
        Strong::from_retain(send::<Raw>(
            NSNotificationCenter::class().ptr(),
            sel!("defaultCenter"),
        ))
    }

    /// `-addObserverForName:object:queue:usingBlock:` — the block-based
    /// observer API, backed by the hand-rolled [`Block1`](crate::blocks::Block1).
    /// The closure receives the `NSNotification*` (`id`).
    ///
    /// # Safety
    /// Calls into the Objective-C runtime. The returned observer must be
    /// removed with `removeObserver` before it is dropped if you stop using
    /// it earlier.
    pub unsafe fn addObserver<F: FnMut(Raw) + 'static>(
        &self,
        name: Ref<NSString>,
        object: Ref<NSObject>,
        queue: Raw,
        block: &Block1<F>,
    ) -> Strong<NSObject> {
        Strong::from_retain(send4::<Raw, Raw, Raw, Raw, Raw>(
            self.as_raw(),
            sel!("addObserverForName:object:queue:usingBlock:"),
            name.as_raw(),
            object.as_raw(),
            queue,
            block.as_raw(),
        ))
    }

    /// `-removeObserver:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn removeObserver(&self, observer: &NSObject) {
        send1::<Raw, ()>(self.as_raw(), sel!("removeObserver:"), observer.as_raw());
    }
}

objc_class!(
    /// `NS::Notification` (`Foundation/NSNotification.hpp`).
    pub struct NSNotification : "NSNotification";
);

impl NSNotification {
    /// `-name`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn name(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("name")))
    }
    /// `-object`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn object(&self) -> Ref<NSObject> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("object")))
    }
    /// `-userInfo`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn userInfo(&self) -> Ref<NSDictionary<NSString, NSObject>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("userInfo")))
    }
}

// ---------------------------------------------------------------------------
// NSBundle
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::Bundle` (`Foundation/NSBundle.hpp`).
    pub struct NSBundle : "NSBundle";
);

impl NSBundle {
    /// `+[NSBundle mainBundle]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn mainBundle() -> Strong<NSBundle> {
        Strong::from_retain(send::<Raw>(NSBundle::class().ptr(), sel!("mainBundle")))
    }

    /// `-URLForResource:withExtension:` (e.g. a `.metal` library file)
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn URLForResource(&self, name: &NSString, extension: &NSString) -> Strong<NSURL> {
        Strong::from_retain(send2::<Raw, Raw, Raw>(
            self.as_raw(),
            sel!("URLForResource:withExtension:"),
            name.as_raw(),
            extension.as_raw(),
        ))
    }

    /// `-bundlePath`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn bundlePath(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("bundlePath")))
    }

    /// `-resourcePath`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn resourcePath(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("resourcePath")))
    }

    /// `-bundleIdentifier`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn bundleIdentifier(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("bundleIdentifier")))
    }

    /// `-infoDictionary`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn infoDictionary(&self) -> Ref<NSDictionary<NSString, NSObject>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("infoDictionary")))
    }

    /// `-load`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn load(&self) -> bool {
        send_bool(self.as_raw(), sel!("load"))
    }

    /// `-isLoaded`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn isLoaded(&self) -> bool {
        send_bool(self.as_raw(), sel!("isLoaded"))
    }
}

// ---------------------------------------------------------------------------
// NSError
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::Error` (`Foundation/NSError.hpp`).
    pub struct NSError : "NSError";
);

impl NSError {
    /// `+[NSError errorWithDomain:code:userInfo:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn error(
        domain: Ref<NSString>,
        code: NSInteger,
        userInfo: Ref<NSDictionary<NSString, NSObject>>,
    ) -> Strong<NSError> {
        Strong::from_retain(send3::<Raw, NSInteger, Raw, Raw>(
            NSError::class().ptr(),
            sel!("errorWithDomain:code:userInfo:"),
            domain.as_raw(),
            code,
            userInfo.as_raw(),
        ))
    }

    /// `-code`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn code(&self) -> NSInteger {
        send::<NSInteger>(self.as_raw(), sel!("code"))
    }

    /// `-domain`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn domain(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("domain")))
    }

    /// `-userInfo`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn userInfo(&self) -> Ref<NSDictionary<NSString, NSObject>> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("userInfo")))
    }

    /// `-localizedDescription`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn localizedDescription(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("localizedDescription")))
    }

    /// `-localizedFailureReason`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn localizedFailureReason(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(self.as_raw(), sel!("localizedFailureReason")))
    }

    /// `-localizedRecoverySuggestion`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn localizedRecoverySuggestion(&self) -> Ref<NSString> {
        Ref::from_raw(send::<Raw>(
            self.as_raw(),
            sel!("localizedRecoverySuggestion"),
        ))
    }
}

// ---------------------------------------------------------------------------
// NSURL
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::URL` (`Foundation/NSURL.hpp`).
    pub struct NSURL : "NSURL";
);

impl NSURL {
    /// `+[NSURL fileURLWithPath:]`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fileURLWithPath(path: &NSString) -> Strong<NSURL> {
        Strong::from_retain(send1::<Raw, Raw>(
            NSURL::class().ptr(),
            sel!("fileURLWithPath:"),
            path.as_raw(),
        ))
    }

    /// `-fileSystemRepresentation`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn fileSystemRepresentation(&self) -> *const c_char {
        send::<*const c_char>(self.as_raw(), sel!("fileSystemRepresentation"))
    }
}

// ---------------------------------------------------------------------------
// NSAutoreleasePool
// ---------------------------------------------------------------------------

objc_class!(
    /// `NS::AutoreleasePool` (`Foundation/NSAutoreleasePool.hpp`) — the class
    /// wrapper around `@autoreleasepool`. For Rust ergonomics prefer
    /// [`crate::objc_abi::autoreleasepool`] (runtime push/pop ABI).
    pub struct NSAutoreleasePool : "NSAutoreleasePool";
);

impl NSAutoreleasePool {
    /// `-drain`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn drain(&self) {
        send::<()>(self.as_raw(), sel!("drain"));
    }

    /// `-addObject:`
    ///
    /// # Safety
    /// Calls into the Objective-C runtime.
    pub unsafe fn addObject(&self, object: &NSObject) {
        send1::<Raw, ()>(self.as_raw(), sel!("addObject:"), object.as_raw());
    }
}
