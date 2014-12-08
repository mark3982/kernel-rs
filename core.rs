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