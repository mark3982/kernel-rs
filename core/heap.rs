/*
*/

use super::Option;

/// A type that represents a uniquely-owned value.
#[lang = "owned_box"]
#[unstable = "custom allocators will add an additional type parameter (with default)"]
pub struct Box<T>(*mut T);

struct HeapChunk {
    offset:     uint,       // offset of chunk in memory
    size:       uint,       // size of entire chunk including data and bitmap
    blocksize:  uint,       // size in bytes governed by bitmap entry
    bmsize:     uint,       // bitmap size in bytes
    dataoffset: uint,       // byte offset of start of data segment
    free:       uint,       // number of bytes currently free
    capacity:   uint,       // maximum number of bytes for allocation
}

struct HeapHdr {
    first:          Option<HeapChunk>
}

static mut HEAPHDR: HeapHdr = HeapHdr {
    first:          Option::None
};


pub fn addchunk(addr: uint, size: uint) {
    bm_addchunk(addr, size, 32);
}

pub fn bm_addchunk(addr: uint, size: uint, blocksize: uint) {
    let hc: HeapChunk = HeapChunk {
        offset:     addr,
        size:       size,
        blocksize:  blocksize,
        bmsize:     size / blocksize,
        dataoffset: (size / blocksize) * 1,
        free:       size / blocksize,
        capacity:   size / blocksize
    };

    unsafe {
        if HEAPHDR.first.is_none() {
            HEAPHDR.first = Option::Some(hc);
            return;
        }
    }
}

pub fn malloc(size: uint, _align: uint) -> *mut u8 {
    unsafe {
        0 as *mut u8
    }
}

#[lang = "exchange_heap"]
#[experimental = "may be renamed; uncertain about custom allocator design"]
pub static HEAP: () = ();

#[lang="exchange_malloc"]
#[inline]
unsafe fn exchange_malloc(size: uint, align: uint) -> *mut u8 {
    0 as *mut u8
}

#[lang="exchange_free"]
#[inline]
unsafe fn exchange_free(ptr: *mut u8, old_size: uint, align: uint) {
    // The most simple heap possible. It does not support
    // deallocation!
}