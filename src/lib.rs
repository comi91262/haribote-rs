#![feature(lang_items)]
#![no_std]
#![feature(asm)]

//extern crate rlibc;

//void HariMain(void)
//{
//	int i; 
//	int* p;
//
//for (i = 0xa0000; i <= 0xaffff; i++) {
//    p = i;
//    *p = i & 0x0f;
//}
//
//	for (;;) {
//	}
//}

#[no_mangle]
pub extern fn rust_main() {
    for i in 0xa0000..0xb0000 {
        let p = i as *mut i32;
        unsafe {
            *p = i & 0x0f;
        }
    }
    loop{}
}

//_write_mem8:	; void write_mem8(int addr, int data);
//		MOV		ECX,[ESP+4]		; [ESP+4]にaddrが入っているのでそれをECXに読み込む
//		MOV		AL,[ESP+8]		; [ESP+8]にdataが入っているのでそれをALに読み込む
//		MOV		[ECX],AL
//		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn write_mem8(addr: u32, data: u32){
//    unsafe {
//        let ecx: u32;
//        let al:   u8;
//        asm!("movl $0,   %ecx" :"={ecx}"(ecx):"0"(addr)::"volatile");
//        asm!("movb $0,   %al"  :"={al}"(al):"0"(data)::"volatile");
//        asm!("movb %al, (%ecx)"::"%al"(al)::"volatile");
//    }
//}


#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

