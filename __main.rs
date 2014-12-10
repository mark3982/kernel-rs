#![feature(lang_items)]
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(asm)]
#![feature(globs)]

extern crate board;
extern crate core;

//static GDT: [u32, ..5] = [0, 1, 2, 3, 4];

//#[start]
//pub fn main(argc: int, argv: *const *const u8) -> int {
//    3
//}

#[cfg(target_arch = "x86")]
struct MultiBootHeader {
    magic:          u32,
    flags:          u32,
    chksum:         i32
}

/*
    This is used on X86. It also supports flat
    binary formats if you change the structure
    and add some fields and a pointer to the
    entry function. QEMU requires this to load
    an ELF32. QEMU supposedly will not correctly
    load an ELF64. If you target X86-64 you will
    likely be producing an ELF64, and if you target
    X86 you will be producing an ELF32.

    http://wiki.osdev.org/Bare_Bones
*/
#[cfg(target_arch = "x86")]
static MBH: MultiBootHeader = MultiBootHeader { 
    magic:          0x1badb002,
    flags:          0x2,
    chksum:         -(0x1badb002 + 0x2)
};

#[cfg(target_arch = "x86")]
#[no_mangle]
pub fn ___entry() {
    unsafe {
        asm!("loop:"); 
        asm!("mov 0xb8000, %eax");
        asm!("mov 0x1, %ecx");
        asm!("mov 0x2, %edx");
        //asm!("movw 0x4107, %bx");
        //asm!("movw %bx, (%eax)");
        //asm!("inc %eax")
        //asm!("movb 65, %al");
        // 0x3f8
        //asm!("mov 0x3f8, %dx");
        //asm!("outb %dx");       
        asm!("jmp loop");
        asm!("movl $0, %esp\n
              jmp x86_kstart" : : "i"(0x20000u))
           
    }
} 

#[cfg(target_arch = "arm")]
#[no_mangle]
pub fn ___entry() {
    // little testing code to make sure CPU is reaching
    // this point by emitting a character to the serial
    // output
    //unsafe {
    //    let mem: *mut u32 = SERIAL_BASE as *mut u32;
    //    *mem = 65 as u32;
    //}

    unsafe {
        asm!("mov sp, $0" : : "i"(0x2000u));
        asm!("b arm_kstart");
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

#[cfg(target_arch = "x86")]
#[no_mangle]
pub fn x86_kstart() {
    loop { }
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub fn arm_kstart() {
    /*
        Print A then B then C to the serial h/w port.
    */
    //let x = box 3u;
    //board::test();

    board::debugchar(65);   
    board::debugchar(66);
    board::debugchar(67);
    board::debugstr("hello");
    loop { }
}