#![no_std]

#![feature(lang_items)]
#![feature(intrinsics)]

extern crate core;
extern crate rlibc;

use core::str::StrSlice;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "fail_fmt"] extern fn fail_fmt() {}

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

/// `fill_slice` fills up a `buffer` with "Hello, world!"
///
/// # DANGER WILL ROBINSON
///
/// This function assumes that you've allocated at least fourteen bytes of memory at `buffer`. If
/// you haven't, bad things may happen.
#[no_mangle]
pub extern "C" fn fill_slice(buffer: *mut u8) {
    unsafe { rlibc::memcpy(buffer, "Hello, world!\0".as_ptr(), 14) };
}

#[no_mangle]
pub extern "C" fn rust_example_init() { }