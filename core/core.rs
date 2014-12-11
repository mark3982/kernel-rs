#![feature(macro_rules)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

#![allow(unused_variables)]

pub mod heap;

#[lang="sized"]
trait Sized {}
#[lang="sync"]
trait Sync {}

#[lang = "stack_exhausted"] 
pub extern fn stack_exhausted() {}
#[lang = "eh_personality"] 
pub extern fn eh_personality() {}

// partial copy from src/libcore/intrinsics.rs
extern "rust-intrinsic" {
    /// Perform a volatile load from the `src` pointer.
    pub fn volatile_load<T>(src: *const T) -> T;
    /// Perform a volatile store to the `dst` pointer.
    pub fn volatile_store<T>(dst: *mut T, val: T);

    // Unsafely transforms a value of one type into a value of another type.
    // Both types must have the same size and alignment, and this guarantee
    // is enforced at compile-time.
    pub fn transmute<T,U>(e: T) -> U;
}

#[macro_export]
macro_rules! panic(
    () => (
    panic!("{}", "explicit panic")
    );
    ($msg:expr) => ({
        static _MSG_FILE_LINE: (&'static str, &'static str, uint) = ($msg, file!(), line!());
        panic(&_MSG_FILE_LINE)
    });
    ($fmt:expr, $($arg:tt)*) => ({
        #[inline(always)]
        fn _run_fmt(fmt: &::std::fmt::Arguments) -> ! {
            static _FILE_LINE: (&'static str, uint) = (file!(), line!());
                panic_fmt(fmt, &_FILE_LINE)
            }
            format_args!(_run_fmt, $fmt, $($arg)*)
        });
)

pub enum Option<T> {
    None,
    Some(T)
}

impl<T> Option<T> {
    fn is_none(&self) -> bool {
        match *self {
            Option::None => true,
            Option::Some(_) => false
        }
    }

    fn unwrap(self) -> T {
        match self {
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
            Option::Some(val) => val
        }
    }
}

#[lang = "iterator"]
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

pub struct StrType {
    ptr:        uint,
    size:       uint

}

pub struct U8Iterator {
    ptr:        StrType,
    ndx:        uint
}

impl Iterator<u8> for U8Iterator {
    fn next(&mut self) -> Option<u8> {
        let c: u8;

        if self.ndx >= self.ptr.size {
            return Option::None;
        }

        unsafe {
            c = *((self.ptr.ptr + self.ndx) as *const u8);
        }

        self.ndx += 1;

        Option::Some(c)
    }
}

pub fn str_u8(s: &str) -> U8Iterator {
    unsafe {
        U8Iterator { ptr: transmute(s), ndx: 0 }
    }
}

#[cold] #[inline(never)] // this is the slow path, always
#[lang="panic"]
pub fn panic(expr_file_line: &(&'static str, &'static str, uint)) -> ! {
    loop { }
}

#[lang = "panic_fmt"] 
//pub fn panic_fmt(fmt: &fmt::Arguments, file_line: &(&'static str, uint)) -> ! {
pub fn panic_fmt() {
    loop { }
}

