#![feature(lang_items)]
#![no_std]
#![feature(asm)]

// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
//#![no_builtins]
// NOTE `linkage = weak` doesn't work for Windows (COFF) or MacOS (MachO). It seems it only works
// for ELF objects.
//#![cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), feature(linkage))]

extern crate rlibc;


#[no_mangle]
pub extern fn rust_main() {
    for i in 0xa0000..0xb0000 {
        write_mem8(i, 15);
    }

    //let hello = b"Hello World!";
    //let color_byte = 0x07;//white  0x1f; // white foreground, blue background
    //let mut hello_colored = [color_byte; 24];
    //for (i, char_byte) in hello.into_iter().enumerate() {
    //    hello_colored[i*2] = *char_byte;
    //}

    //// write `Hello World!` to the center of the VGA text buffer
    //let buffer_ptr = (0xa0000 + 100 + 100) as *mut _;
    ////let buffer_ptr = (0xa00000) as *mut _;
    //unsafe { *buffer_ptr = hello_colored };
    loop{}
}

//void io_hlt(void);
//void write_mem8(int addr, int data);
//
//void HariMain(void)
//{
//	int i; /* Ïé¾BiÆ¢¤ÏÍA32rbgÌ®^ */
//
//	for (i = 0xa0000; i <= 0xaffff; i++) {
//		write_mem8(i, 15); /* MOV BYTE [i],15 */
//	}
//
//	for (;;) {
//		io_hlt();
//	}
//}



#[no_mangle]
#[inline(never)]
pub extern fn write_mem8(addr: u32, data: u32){
    unsafe {
        let mut ecx: u32;
        let al:   u8;
        asm!("movl $0, %ecx":"={ecx}"(ecx)   :"0"(addr));
        asm!("movb $0, %al" :"={al}"(al):"0"(data));
        asm!("mov %al, (%ecx)":"={ecx}"(ecx):"%al"(al));
    }
}

//; naskfunc
//; TAB=4
//
//[FORMAT "WCOFF"]				; オブジェクトファイルを作るモード	
//[INSTRSET "i486p"]				; 486の命令まで使いたいという記述
//[BITS 32]						; 32ビットモード用の機械語を作らせる
//[FILE "naskfunc.nas"]			; ソースファイル名情報
//
//		GLOBAL	_io_hlt,_write_mem8
//
//[SECTION .text]
//
//_io_hlt:	; void io_hlt(void);
//		HLT
//		RET
//
//_write_mem8:	; void write_mem8(int addr, int data);
//		MOV		ECX,[ESP+4]		; [ESP+4]にaddrが入っているのでそれをECXに読み込む
//		MOV		AL,[ESP+8]		; [ESP+8]にdataが入っているのでそれをALに読み込む
//		MOV		[ECX],AL
//		RET

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
