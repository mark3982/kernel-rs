#![feature(lang_items)]
#![no_std]

#[lang="sized"]
trait Sized {}
#[lang="sync"]
trait Sync {}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[lang = "exchange_heap"]
#[experimental = "may be renamed; uncertain about custom allocator design"]
pub static HEAP: () = ();

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