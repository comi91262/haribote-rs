#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![no_std]
#![feature(asm)]

#[no_mangle]
fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[no_mangle]
#[start]
pub extern fn init_os() {
    loop {
        hlt()
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }
