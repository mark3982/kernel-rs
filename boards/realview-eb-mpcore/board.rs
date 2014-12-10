/*
    This contains the board code for the ARM realview-eb-mpcore. It
    contains everything specific to the board that is abstracted
    away from the kernel code. 

    It is produced at this time as a Rust library which is then used by
    the kernel when it is built.
*/
#![no_std]

extern crate core;

// this board has a serial port and we use it by default for debugging information
const SERIAL_BASE: u32 = 0x10009000;
const SERIAL_FLAG_REGISTER: u32 = 0x18;
const SERIAL_BUFFER_FULL: u32 = 1 << 15;

// output a character to whatever debug device; the debug device may or
// may not exist and this function is allowed to simply discard the result
// i hope that maybe i can use the optimizer to optimize out debugging code
// when this function becomes empty and does nothing!?
pub fn debugchar(c: u8) {
    unsafe {
        let mem: *mut u32 = (SERIAL_BASE + SERIAL_FLAG_REGISTER) as *mut u32;

        // a read should happen each iteration of the loop and the value
        // should be checked for the full flag and if the buffer is full
        // then we should continue spinning until it is empty
        while core::volatile_load(mem as *const u32) & SERIAL_BUFFER_FULL > 0 {}

        let mem: *mut u32 = SERIAL_BASE as *mut u32;

        *mem = c as u32;
    }
}

pub fn debugstr(s: &str) {
    for c in core::str_u8(s) {
        debugchar(c);
    }
}