#![feature(lang_items)]
#![no_std]
// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
//#![no_builtins]
// NOTE `linkage = weak` doesn't work for Windows (COFF) or MacOS (MachO). It seems it only works
// for ELF objects.
//#![cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), feature(linkage))]

extern crate rlibc;


#[no_mangle]
pub extern fn rust_main() {
    let hello = b"Hello World!";
    let color_byte = 0x07;//white  0x1f; // white foreground, blue background
    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xa0000 + 100 + 100) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



//#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
//#[no_mangle]
//pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
//                            n: usize) -> *mut u8 {
//    let mut i = 0;
//    while i < n {
//        *dest.offset(i as isize) = *src.offset(i as isize);
//        i += 1;
//    }
//    return dest;
//}
//
//#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
//#[no_mangle]
//pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
//                             n: usize) -> *mut u8 {
//    if src < dest as *const u8 { // copy from end
//        let mut i = n;
//        while i != 0 {
//            i -= 1;
//            *dest.offset(i as isize) = *src.offset(i as isize);
//        }
//    } else { // copy from beginning
//        let mut i = 0;
//        while i < n {
//            *dest.offset(i as isize) = *src.offset(i as isize);
//            i += 1;
//        }
//    }
//    return dest;
//}
//
//#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
//#[no_mangle]
//pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
//    let mut i = 0;
//    while i < n {
//        *s.offset(i as isize) = c as u8;
//        i += 1;
//    }
//    return s;
//}
//
//#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
//#[no_mangle]
//pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
//    let mut i = 0;
//    while i < n {
//        let a = *s1.offset(i as isize);
//        let b = *s2.offset(i as isize);
//        if a != b {
//            return a as i32 - b as i32
//        }
//        i += 1;
//    }
//    return 0;
//}
//
