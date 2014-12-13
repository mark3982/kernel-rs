#![feature(lang_items)]
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(asm)]
#![feature(globs)]

extern crate board;
extern crate core;

const SERIAL_BASE: u32 = 0x10009000;

#[cfg(target_arch = "arm")]
#[no_mangle]
pub fn _start() {
    // little testing code to make sure CPU is reaching
    // this point by emitting a character to the serial
    // output
    //unsafe {
    //    let mem: *mut u32 = SERIAL_BASE as *mut u32;
    //    *mem = 65 as u32;
    //}

    //loop { }
    
    unsafe {
        asm!("mov sp, $0" : : "i"(0x2000u));
        asm!("b kentry");
    }
    
    // make sure we do not fly by if something bad happens
    loop { }
}

#[no_mangle]
pub extern fn memset() { loop {} }
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() { loop {} }
#[no_mangle]
pub extern fn __morestack() { loop {} }
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() { loop {} }

#[no_mangle]
pub extern fn kentry() {
    /*
        Print A then B then C to the serial h/w port.
    */
    //let x = box 3u;

    //board::test();

    board::debugchar(65);
    board::debugchar(66);
    board::debugchar(67);
    //board::debugstr("hello");

    //board::init();

    loop { }
}