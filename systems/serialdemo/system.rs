#![feature(lang_items)]
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(asm)]
#![feature(globs)]

extern crate board;
extern crate core;

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