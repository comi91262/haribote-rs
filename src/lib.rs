#![feature(lang_items)]
#![no_std]
#![feature(asm)]

const COL8_000000: u8 = 0;
const COL8_FF0000: u8 = 1;
const COL8_00FF00: u8 = 2;
const COL8_FFFF00: u8 = 3;
const COL8_0000FF: u8 = 4;
const COL8_FF00FF: u8 = 5;
const COL8_00FFFF: u8 = 6;
const COL8_FFFFFF: u8 = 7;
const COL8_C6C6C6: u8 = 8;
const COL8_840000: u8 = 9;
const COL8_008400: u8 = 10;
const COL8_848400: u8 = 11;
const COL8_000084: u8 = 12;
const COL8_840084: u8 = 13;
const COL8_008484: u8 = 14;
const COL8_848484: u8 = 15;


//struct BOOTINFO {
//	char cyls, leds, vmode, reserve;
//	short scrnx, scrny;
//	char *vram;
//};
struct BOOTINFO {
    cyls: u8,
    leds: u8,
    vmode: u8,
    reserve: u8,
    scrnx: u16,
    scrny: u16,
    vram: *const usize
}

//void HariMain(void)
//{
//	char *vram;
//	int xsize, ysize;
//
//	init_palette();
//	init_screen(binfo->vram, binfo->scrnx, binfo->scrny);
//
//
//	for (;;) {
//	}
//}
#[no_mangle]
#[warn(unused_assignments)] 
pub extern fn rust_main() {

    let binfo = 0x0ff0 as *const (BOOTINFO);

    init_palette();
    unsafe {
        //0x0ff8
        init_screen(
            *(0x0ff8 as *const u32) as *const u32, //*((*binfo).vram)!
            (*binfo).scrnx as u32,
            (*binfo).scrny as u32);
        putfont8(
            *(0x0ff8 as *const u32) as *const u32, //*((*binfo).vram)!
            (*binfo).scrnx as u32,
            8, 
            8,
            COL8_FFFFFF,
            "ABC 123");
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
    static RGB: [u8; 16 * 3] = [    // !! because of --release, this program works.
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
    let p = &RGB as *const u8;
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

//void putfont8(char *vram, int xsize, int x, int y, char c, char *font)
//{
//	int i;
//	char *p, d /* data */;
//	for (i = 0; i < 16; i++) {
//		p = vram + (y + i) * xsize + x;
//		d = font[i];
//		if ((d & 0x80) != 0) { p[0] = c; }
//		if ((d & 0x40) != 0) { p[1] = c; }
//		if ((d & 0x20) != 0) { p[2] = c; }
//		if ((d & 0x10) != 0) { p[3] = c; }
//		if ((d & 0x08) != 0) { p[4] = c; }
//		if ((d & 0x04) != 0) { p[5] = c; }
//		if ((d & 0x02) != 0) { p[6] = c; }
//		if ((d & 0x01) != 0) { p[7] = c; }
//	}
//	return;
//}
fn putfont8(vram: *const u32, xsize: u32, x: u32, y: u32, c: u8, font: &str){
   

    static font_A: [u8; 16] = [
        0x00, 0x18, 0x18, 0x18, 0x18, 0x24, 0x24, 0x24,
        0x24, 0x7e, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00
    ];

    let font = font_A.as_ptr();

    unsafe {
        for i in 0..16 {
            let p = vram as u32 + (y + i) * xsize + x;
            let d = *((font as u32 + i) as *const u8);
            if (d & 0x80) != 0 { *(p as *mut u8) = c; }
            if (d & 0x40) != 0 { *((p as u32 + 1) as *mut u8) = c; }
            if (d & 0x20) != 0 { *((p as u32 + 2) as *mut u8) = c; }
            if (d & 0x10) != 0 { *((p as u32 + 3) as *mut u8) = c; }
            if (d & 0x08) != 0 { *((p as u32 + 4) as *mut u8) = c; }
            if (d & 0x04) != 0 { *((p as u32 + 5) as *mut u8) = c; }
            if (d & 0x02) != 0 { *((p as u32 + 6) as *mut u8) = c; }
            if (d & 0x01) != 0 { *((p as u32 + 7) as *mut u8) = c; }
        }
    }
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
