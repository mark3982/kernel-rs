/*
*/
#![allow(dead_code)]

use super::*;

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
    chunks:         [Option<HeapChunk>, ..10]
}

static mut HEAPHDR: HeapHdr = HeapHdr {
    chunks:         [Option::None, ..10]
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
        let mut i: uint = 0u;
        while i < HEAPHDR.chunks.len() {
            if HEAPHDR.chunks[i].is_none() {
                HEAPHDR.chunks[i] = Option::Some(hc);
                return;
            }
            i += 1;
        }
    }

    //panic!("could not add bitmap heap chunk");
    loop { }
}

fn bm_divu(a: uint, b: uint) -> uint {
    if a % b == 0 {
        return a / b;
    }
    a / b + 1
}

//
// Set blocks to specified value. This is going to be called not 
// only by the allocation code but also externally to specify 
// certain regions as used.
//
fn bm_set(hc: HeapChunk, offset: uint, size: uint, val: u8) {
    let mut bmi: uint = ((offset - hc.offset) - hc.dataoffset) / hc.blocksize;
    let bme: uint = bm_divu(size, hc.blocksize) + bmi;

    while bmi < bme {
        unsafe {
            *((hc.offset + bmi) as *mut u8) = val;
        }
        bmi += 1;
    }
}

//
// Try to allocate from a heap chunk.
//
fn bm_mallocfrom(hc: HeapChunk, size: uint, align: uint) -> Option<*mut u8> {
    let mut sdx: uint;      // search index
    let mut fnd: uint;      // find count
    let mut srt: uint;      // start index

    unsafe {
        sdx = 0;
        fnd = 0;
        srt = 0; 

        while sdx < hc.bmsize {
            // keep track of how mny free blocks we have found
            if *((hc.offset + sdx) as *const u8) == 1 {
                fnd = 0;
                srt = sdx + 1;
                continue;
            }
            fnd += 1;
            // determine if we have enough blocks for an allocation
            // and then set the blocks to used and return a pointer
            // to the block of dat
            if fnd * hc.blocksize >= size {
                let soff: uint = hc.offset + hc.dataoffset + hc.blocksize * srt;
                bm_set(hc, soff, size, 1);
                return Option::Some(soff as *mut u8);
            }
        }
    }

    Option::None
}

//
// A wrapper function which checks each heap chunk and calls the actual
// function which will attempt an allocation from the heap chunk.
//
fn bm_malloc(size: uint, align: uint) -> *mut u8 {
    unsafe {
        let ret: Option<*mut u8>;
        let mut i: uint = 0u;
        while i < HEAPHDR.chunks.len() {
            match HEAPHDR.chunks[i] {
                Option::None => break,
                Option::Some(hc) => {
                    if hc.free >= size {
                        match bm_mallocfrom(hc, size, align) {
                            Option::None => break,
                            Option::Some(ptr) => return ptr
                        } 
                    }
                }
            }
        }

        0 as *mut u8
    }
}

pub fn malloc(size: uint, align: uint) -> *mut u8 {
    bm_malloc(size, align)
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