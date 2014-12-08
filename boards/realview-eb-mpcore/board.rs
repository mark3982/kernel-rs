/*
    This contains the board code for the ARM realview-eb-mpcore. It
    contains everything specific to the board that is abstracted
    away from the kernel code. 

    It is produced at this time as a Rust library which is then used by
    the kernel when it is built.
*/
#![no_std]

extern crate core;

pub fn test() -> int {
    88
}