#![feature(lang_items)]
#![no_std]
#![feature(asm)]

//void HariMain(void)
//{
//	char *vram;
//	int xsize, ysize;
//
//	init_palette();
//	vram = (char *) 0xa0000;
//	xsize = 320;
//	ysize = 200;
//
//	boxfill8(vram, xsize, COL8_008484,  0,         0,          xsize -  1, ysize - 29);
//	boxfill8(vram, xsize, COL8_C6C6C6,  0,         ysize - 28, xsize -  1, ysize - 28);
//	boxfill8(vram, xsize, COL8_FFFFFF,  0,         ysize - 27, xsize -  1, ysize - 27);
//	boxfill8(vram, xsize, COL8_C6C6C6,  0,         ysize - 26, xsize -  1, ysize -  1);
//
//	boxfill8(vram, xsize, COL8_FFFFFF,  3,         ysize - 24, 59,         ysize - 24);
//	boxfill8(vram, xsize, COL8_FFFFFF,  2,         ysize - 24,  2,         ysize -  4);
//	boxfill8(vram, xsize, COL8_848484,  3,         ysize -  4, 59,         ysize -  4);
//	boxfill8(vram, xsize, COL8_848484, 59,         ysize - 23, 59,         ysize -  5);
//	boxfill8(vram, xsize, COL8_000000,  2,         ysize -  3, 59,         ysize -  3);
//	boxfill8(vram, xsize, COL8_000000, 60,         ysize - 24, 60,         ysize -  3);
//
//	boxfill8(vram, xsize, COL8_848484, xsize - 47, ysize - 24, xsize -  4, ysize - 24);
//	boxfill8(vram, xsize, COL8_848484, xsize - 47, ysize - 23, xsize - 47, ysize -  4);
//	boxfill8(vram, xsize, COL8_FFFFFF, xsize - 47, ysize -  3, xsize -  4, ysize -  3);
//	boxfill8(vram, xsize, COL8_FFFFFF, xsize -  3, ysize - 24, xsize -  3, ysize -  3);
//
//	for (;;) {
// = io_hlt();
//	}
//}

struct BOOTINFO {
    cyls: u8,
    leds: u8,
    vmode: u8,
    reserve: u8,
    scrnx: u16,
    scrny: u16,
    vram: u32
}

#[no_mangle]
#[warn(unused_assignments)] 
pub extern fn rust_main() {

    let binfo = 0x0ff0 as *const (BOOTINFO);

    init_palette();



	//binfo_scrnx = (short *) 0x0ff4;
	//binfo_scrny = (short *) 0x0ff6;
	//binfo_vram = (int *) 0x0ff8;
	//xsize = *binfo_scrnx;
	//ysize = *binfo_scrny;
	//vram = (char *) *binfo_vram;

	//pushl	$655360
	//init_screen(vram, xsize, ysize);
// let origin = Point { x: 0, y: 0 }; // origin: Point
    //let vram = 0xa0000 as *const u32;
    unsafe {
        let x = (*binfo).scrnx as u32;
        let y = (*binfo).scrny as u32;
        let vram = ((*binfo + 8).vram) as *const u32;
        init_screen(*vram as *const u32, x, y);
    }


    loop{}
}

//void init_palette(void)
//{
//	static unsigned char table_rgb[16 * 3] = {
//		0x00, 0x00, 0x00,	/*  0:黒 */
//		0xff, 0x00, 0x00,	/*  1:明るい赤 */
//		0x00, 0xff, 0x00,	/*  2:明るい緑 */
//		0xff, 0xff, 0x00,	/*  3:明るい黄色 */
//		0x00, 0x00, 0xff,	/*  4:明るい青 */
//		0xff, 0x00, 0xff,	/*  5:明るい紫 */
//		0x00, 0xff, 0xff,	/*  6:明るい水色 */
//		0xff, 0xff, 0xff,	/*  7:白 */
//		0xc6, 0xc6, 0xc6,	/*  8:明るい灰色 */
//		0x84, 0x00, 0x00,	/*  9:暗い赤 */
//		0x00, 0x84, 0x00,	/* 10:暗い緑 */
//		0x84, 0x84, 0x00,	/* 11:暗い黄色 */
//		0x00, 0x00, 0x84,	/* 12:暗い青 */
//		0x84, 0x00, 0x84,	/* 13:暗い紫 */
//		0x00, 0x84, 0x84,	/* 14:暗い水色 */
//		0x84, 0x84, 0x84	/* 15:暗い灰色 */
//	};
//	set_palette(0, 15, table_rgb);
//	return;
//
//	/* static char 命令は、データにしか使えないけどDB命令相当 */
//}

#[no_mangle]
#[inline(never)]
pub extern fn init_palette(){

    set_palette(0, 15);//, &table_rgb);

}

//void set_palette(int start, int end, unsigned char *rgb)
//{
//	int i, eflags;
//	eflags = io_load_eflags();	/* 割り込み許可フラグの値を記録する */
//	io_cli(); 					/* 許可フラグを0にして割り込み禁止にする */
//	io_out8(0x03c8, start);
//	for (i = start; i <= end; i++) {
//		io_out8(0x03c9, rgb[0] / 4);
//		io_out8(0x03c9, rgb[1] / 4);
//		io_out8(0x03c9, rgb[2] / 4);
//		rgb += 3;
//	}
//	io_store_eflags(eflags);	/* 割り込み許可フラグを元に戻す */
//	return;
//}
#[no_mangle]
#[inline(never)]
pub extern fn set_palette(start: u8, end: u8)//, rgb: &[u8; 48])
{
    const rgb: [u8; 16 * 3] = [
        0x00, 0x00, 0x00,	/*  0:黒 */
        0xff, 0x00, 0x00,	/*  1:明るい赤 */
        0x00, 0xff, 0x00,	/*  2:明るい緑 */
        0xff, 0xff, 0x00,	/*  3:明るい黄色 */
        0x00, 0x00, 0xff,	/*  4:明るい青 */
        0xff, 0x00, 0xff,	/*  5:明るい紫 */
        0x00, 0xff, 0xff,	/*  6:明るい水色 */
        0xff, 0xff, 0xff,	/*  7:白 */
        0xc6, 0xc6, 0xc6,	/*  8:明るい灰色 */
        0x84, 0x00, 0x00,	/*  9:暗い赤 */
        0x00, 0x84, 0x00,	/* 10:暗い緑 */
        0x84, 0x84, 0x00,	/* 11:暗い黄色 */
        0x00, 0x00, 0x84,	/* 12:暗い青 */
        0x84, 0x00, 0x84,	/* 13:暗い紫 */
        0x00, 0x84, 0x84,	/* 14:暗い水色 */
        0x84, 0x84, 0x84	/* 15:暗い灰色 */
    ];

    let eflags = io_load_eflags();	/* 割り込み許可フラグの値を記録する */
    io_cli();                   	/* 許可フラグを0にして割り込み禁止にする */
    io_out8(0x03c8, start);
    let p = &rgb as *const u8;
    for i in 0..48 {
        unsafe {
            io_out8(0x03c9,  *p.offset(i as isize) / 4);
        }
    }
//    for i in start..(end + 1) {
//       unsafe {
//           io_out8(0x03c9,  *p.offset(3 as isize));
//           io_out8(0x03c9,  *p.offset(4 as isize));
//           io_out8(0x03c9,  *p.offset(5 as isize));
//           //io_out8(0x03c9, *p.offset((3 * i    ) as isize) );
//           //io_out8(0x03c9, *p.offset((3 * i + 1) as isize) );
//           //io_out8(0x03c9, *p.offset((3 * i + 2) as isize) );
//       }
//    }
//
	io_store_eflags(eflags);	/* 割り込み許可フラグを元に戻す */
}

//void boxfill8(unsigned char *vram, int xsize, unsigned char c, int x0, int y0, int x1, int y1)
//{
//	int x, y;
//	for (y = y0; y <= y1; y++) {
//		for (x = x0; x <= x1; x++)
//			vram[y * xsize + x] = c;
//	}
//	return;
//}
#[no_mangle]
#[inline(never)]
pub extern fn boxfill8(vram: *const u32, xsize: u32, c: u8, x0: u32, y0: u32, x1: u32, y1: u32)
{
    let p = vram as u32;
    for y in y0..(y1 + 1) {
        for x in x0..(x1 + 1) {
            unsafe {
                *((p + y * xsize + x) as *mut u8) = c;
            }
        }
    }
}

//_io_cli:	; void io_cli(void);
//		CLI
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn io_cli(){
    unsafe {
        asm!("cli");
        asm!("ret");
    }
}

//_io_sti:	; void io_sti(void);
//		STI
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn io_sti(){
    unsafe {
        asm!("sti");
        asm!("ret");
    }
}



//_io_out8:	; void io_out8(int port, int data);
//		MOV		EDX,[ESP+4]		; port
//		MOV		AL,[ESP+8]		; data
//		OUT		DX,AL
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn io_out8(port: u32, data: u8){
    let edx: u32;
    let al : u8;
    unsafe {
        asm!("movl $0,   %edx" :"={edx}"(edx):"0"(port)::"volatile");
        asm!("movb $0,   %al"  :"={al}"(al)  :"0"(data)::"volatile");
        asm!("out  %al,  %dx");
        asm!("ret");

    }
}

//_io_load_eflags:	; int io_load_eflags(void);
//		PUSHFD		; PUSH EFLAGS Æ¢¤Ó¡
//		POP		EAX
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn io_load_eflags() -> u32 {
    let eax;
    unsafe {
        asm!("pushfd");
        asm!("pop %eax":"={eax}"(eax));
    }
    eax
}

//_io_store_eflags:	; void io_store_eflags(int eflags);
//		MOV		EAX,[ESP+4]
//		PUSH	EAX
//		POPFD		; POP EFLAGS Æ¢¤Ó¡
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn io_store_eflags(eflags: u32){
    let eax: u32;
    unsafe {
        asm!("movl $0, %eax":"={eax}"(eax):"0"(eflags)::"volatile");
        asm!("push %eax");
        asm!("popfd");
        asm!("ret");
    }
}

#[no_mangle]
#[inline(never)]
pub extern fn init_screen(vram: *const u32, x: u32, y: u32)
{

    let COL8_000000 = 0; let COL8_FF0000 = 1; let COL8_00FF00 = 2; let COL8_FFFF00 = 3;
    let COL8_0000FF = 4; let COL8_FF00FF = 5; let COL8_00FFFF = 6; let COL8_FFFFFF = 7;
    let COL8_C6C6C6 = 8; let COL8_840000 = 9; let COL8_008400 = 10;let COL8_848400 = 11;
    let COL8_000084 = 12;let COL8_840084 = 13;let COL8_008484 = 14;let COL8_848484 = 15;

	boxfill8(vram, x, COL8_008484,  0,         0,      x -  1, y - 29);
	boxfill8(vram, x, COL8_C6C6C6,  0,         y - 28, x -  1, y - 28);
	boxfill8(vram, x, COL8_FFFFFF,  0,         y - 27, x -  1, y - 27);
	boxfill8(vram, x, COL8_C6C6C6,  0,         y - 26, x -  1, y -  1);

	boxfill8(vram, x, COL8_FFFFFF,  3,         y - 24, 59,         y - 24);
	boxfill8(vram, x, COL8_FFFFFF,  2,         y - 24,  2,         y -  4);
	boxfill8(vram, x, COL8_848484,  3,         y -  4, 59,         y -  4);
	boxfill8(vram, x, COL8_848484, 59,         y - 23, 59,         y -  5);
	boxfill8(vram, x, COL8_000000,  2,         y -  3, 59,         y -  3);
	boxfill8(vram, x, COL8_000000, 60,         y - 24, 60,         y -  3);

	boxfill8(vram, x, COL8_848484, x - 47, y - 24, x -  4, y - 24);
	boxfill8(vram, x, COL8_848484, x - 47, y - 23, x - 47, y -  4);
	boxfill8(vram, x, COL8_FFFFFF, x - 47, y -  3, x -  4, y -  3);
	boxfill8(vram, x, COL8_FFFFFF, x -  3, y - 24, x -  3, y -  3);

}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}


//_io_in8:	; int io_in8(int port);
//		MOV		EDX,[ESP+4]		; port
//		MOV		EAX,0
//		IN		AL,DX
//		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_in8(port: u32){
//    let esp: u32;
//    unsafe {
//        asm!("movl $0,  4(%esp)" :"={esp}"(esp):"0"(port)::"volatile");
//        asm!("movl $$0, %eax"    ::::"volatile");
//        asm!("in %al, %dx");
//        asm!(ret);
//    }
//}
//_io_out16:	; void io_out16(int port, int data);
//		MOV		EDX,[ESP+4]		; port
//		MOV		EAX,[ESP+8]		; data
//		OUT		DX,AX
//		RET
//
//_io_out32:	; void io_out32(int port, int data);
//		MOV		EDX,[ESP+4]		; port
//		MOV		EAX,[ESP+8]		; data
//		OUT		DX,EAX
//		RET
//#[no_mangle]
//#[inline(never)]
//pub extern pub extern fn write_mem8(addr: u32, data: u32){
//    unsafe {
//        let ecx: u32;
//        let al:   u8;
//        asm!("movl $0,   %ecx" :"={ecx}"(ecx):"0"(addr)::"volatile");
//        asm!("movb $0,   %al"  :"={al}"(al):"0"(data)::"volatile");
//        asm!("movb %al, (%ecx)"::"%al"(al)::"volatile");
//    }
//}
