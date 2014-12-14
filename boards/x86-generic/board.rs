/*
    This contains the board code for the ARM realview-eb-mpcore. It
    contains everything specific to the board that is abstracted
    away from the kernel code. 

    It is produced at this time as a Rust library which is then used by
    the kernel when it is built.
*/
#![no_std]
#![feature(asm)]
#![allow(unused_variables)]

extern crate core;

#[no_mangle]
pub extern fn board_panic() {
    //debugstr("board panic!");
    loop { }
}

fn outport8(port: u16, data: u8) {
    unsafe {
        asm!("
            mov $0, %dx
            mov $1, %al
            outb %al, %dx
        " : : "r"(port), "r"(data) : "al", "dx")
    }
}

pub fn inport8(port: u16) -> u8 {
    let out: u8;

    unsafe {
        asm!("
            mov $1, %dx
            inb %dx, %al
            mov %al, $0
        " : "=r"(out) : "r"(port) : "al", "dx");
    }

    out
}


pub fn debugchar(c: u8) {
    while (inport8(0x3f8 + 0x5) & 0x60) != 0x60 { }
    outport8(0x3f8, c);
}

pub fn debugstr(s: &str) {
    for c in core::str_u8(s) {
        debugchar(c);
    }
}

pub fn init() {
}