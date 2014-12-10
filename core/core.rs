#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

#![allow(unused_variables)]

#[lang="sized"]
trait Sized {}
#[lang="sync"]
trait Sync {}

#[lang = "stack_exhausted"] 
pub extern fn stack_exhausted() {}
#[lang = "eh_personality"] 
pub extern fn eh_personality() {}
#[lang = "panic_fmt"] 
pub fn panic_fmt() -> ! { loop {} }

#[lang = "exchange_heap"]
#[experimental = "may be renamed; uncertain about custom allocator design"]
pub static HEAP: () = ();

// partial copy from src/libcore/intrinsics.rs
extern "rust-intrinsic" {
    /// Perform a volatile load from the `src` pointer.
    pub fn volatile_load<T>(src: *const T) -> T;
    /// Perform a volatile store to the `dst` pointer.
    pub fn volatile_store<T>(dst: *mut T, val: T);
}

/// A type that represents a uniquely-owned value.
#[lang = "owned_box"]
#[unstable = "custom allocators will add an additional type parameter (with default)"]
pub struct Box<T>(*mut T);

struct Global {
    heapoffset:     uint,
    curheapndx:     uint
}

static mut GLOBAL: Global = Global {
    heapoffset:     0,
    curheapndx:     0
};

#[lang="exchange_malloc"]
#[inline]
unsafe fn exchange_malloc(size: uint, align: uint) -> *mut u8 {
    // The most simple heap possible!
    let ptr: uint;
    ptr = GLOBAL.heapoffset + GLOBAL.curheapndx;
    GLOBAL.curheapndx += size;

    ptr as *mut u8
}

#[lang="exchange_free"]
#[inline]
unsafe fn exchange_free(ptr: *mut u8, old_size: uint, align: uint) {
    // The most simple heap possible. It does not support
    // deallocation!
}