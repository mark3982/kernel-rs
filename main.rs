#![feature(lang_items)]
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(asm)]
#![feature(globs)]

extern crate board;
extern crate core;

use core::*;

static GDT: [u32, ..5] = [0, 1, 2, 3, 4];

//#[start]
//pub fn main(argc: int, argv: *const *const u8) -> int {
//    3
//}

#[no_mangle]
pub fn entry() {
    unsafe {
        asm!("mov sp, $0" : : "i"(0x2000u));
    }

    kstart();

    unsafe {
        asm!("b kstart");
    }
}

#[no_mangle]
pub extern fn __morestack() { loop {} }
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }

const SERIAL_BASE: u32 = 0x10009000;
const SERIAL_FLAG_REGISTER: u32 = 0x18;
const SERIAL_BUFFER_FULL: u32 = 1 << 15;

fn kserdbg_putc(c: u8) {
    unsafe {
        let mem: *mut u32 = (SERIAL_BASE + SERIAL_FLAG_REGISTER) as *mut u32;

        while (*mem & SERIAL_BUFFER_FULL) == 0 {}

        let mem: *mut u32 = SERIAL_BASE as *mut u32;

        *mem = c as u32;
    }
}

#[no_mangle]
fn kstart() {
    /*
        Print A then B then C to the serial h/w port.
    */
    let x: Box<uint> = box 3u;

    board::test();

    kserdbg_putc(65);   
    kserdbg_putc(66);
    kserdbg_putc(67);
    loop { }
}