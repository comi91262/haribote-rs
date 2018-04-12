#![feature(lang_items)]
#![no_std]
#![feature(asm)]

extern crate rlibc;

const COL8_000000: u8 = 0;
const _COL8_FF0000: u8 = 1;
const _COL8_00FF00: u8 = 2;
const _COL8_FFFF00: u8 = 3;
const _COL8_0000FF: u8 = 4;
const _COL8_FF00FF: u8 = 5;
const _COL8_00FFFF: u8 = 6;
const COL8_FFFFFF: u8 = 7;
const COL8_C6C6C6: u8 = 8;
const _COL8_840000: u8 = 9;
const _COL8_008400: u8 = 10;
const _COL8_848400: u8 = 11;
const _COL8_000084: u8 = 12;
const _COL8_840084: u8 = 13;
const COL8_008484: u8 = 14;
const COL8_848484: u8 = 15;


struct BOOTINFO {
    _cyls: u8,
    _leds: u8,
    _vmode: u8,
    _reserve: u8,
    scrnx: u16,
    scrny: u16,
    vram: *const usize
}

struct SEGMENT_DESCRIPTOR {
    limit_low: u16,
    base_low:  u16,
    base_mid:  u8,
    access_right: u8,
    limit_high: u8,
    base_high: u8
}

struct GATE_DESCRIPTOR {
    offset_low:  u16,
    selector:    u16,
    dw_count:     u8,
    access_right: u8,
    offset_high: u16
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

    let mut mcursor: [u8; 256] = [0; 256];
    let binfo = 0x0ff0 as *const (BOOTINFO);

    init_palette();
	init_gdtidt();
    unsafe {
        let mx = ((*binfo).scrnx as u32 - 16) / 2;
        let my = ((*binfo).scrny as u32 - 28 - 16) / 2;
        //0x0ff8
        init_screen(
            *(0x0ff8 as *const u32) as *const u32, //*((*binfo).vram)
            (*binfo).scrnx as u32,
            (*binfo).scrny as u32);
        put_font_big_a(
            *(0x0ff8 as *const u32) as *const u32, //*((*binfo).vram)
            (*binfo).scrnx as u32,
            8, 
            8,
            COL8_FFFFFF);
        init_mouse_cursor8(mcursor.as_mut_ptr(), COL8_008484);
        putblock8_8(*(0x0ff8 as *const u32) as *const u32, //*((*binfo).vram)
                    (*binfo).scrnx as u32,
                    16,
                    16,
                    mx,
                    my, 
                    mcursor.as_mut_ptr(),
                    16);
    }


    loop{}
}

//void init_palette(void)
//{
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
    let rgb: [u8; 16 * 3] = [   
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
    let p = rgb.as_ptr();
    for i in start..(end + 1) {
        unsafe {
            io_out8(0x03c9,  *p.offset((3 * i    ) as isize) / 4);
            io_out8(0x03c9,  *p.offset((3 * i + 1) as isize) / 4);
            io_out8(0x03c9,  *p.offset((3 * i + 2) as isize) / 4);
        }
    }

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


#[no_mangle]
#[inline(never)]
pub extern fn init_screen(vram: *const u32, x: u32, y: u32)
{


	boxfill8(vram, x, COL8_008484,  0, 0,      x -  1, y - 29);
	boxfill8(vram, x, COL8_C6C6C6,  0, y - 28, x -  1, y - 28);
	boxfill8(vram, x, COL8_FFFFFF,  0, y - 27, x -  1, y - 27);
	boxfill8(vram, x, COL8_C6C6C6,  0, y - 26, x -  1, y -  1);

	boxfill8(vram, x, COL8_FFFFFF,  3, y - 24, 59, y - 24);
	boxfill8(vram, x, COL8_FFFFFF,  2, y - 24,  2, y -  4);
	boxfill8(vram, x, COL8_848484,  3, y -  4, 59, y -  4);
	boxfill8(vram, x, COL8_848484, 59, y - 23, 59, y -  5);
	boxfill8(vram, x, COL8_000000,  2, y -  3, 59, y -  3);
	boxfill8(vram, x, COL8_000000, 60, y - 24, 60, y -  3);

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
fn put_font_big_a(vram: *const u32, xsize: u32, x: u32, y: u32, c: u8){
   
    let font_big_a: [u8; 16] = [
        0x00, 0x18, 0x18, 0x18, 0x18, 0x24, 0x24, 0x24,
        0x24, 0x7e, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00
    ];

    let font = font_big_a.as_ptr();

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

fn init_mouse_cursor8(mouse: *mut u8, bc: u8){

    let cursor = [
        [b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'*',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'0',b'*',b'*',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'0',b'*',b'.',b'.',b'*',b'0',b'0',b'0',b'*',b'.',b'.',b'.',b'.'],
        [b'*',b'0',b'0',b'*',b'.',b'.',b'.',b'.',b'*',b'0',b'0',b'0',b'*',b'.',b'.',b'.'],
        [b'*',b'0',b'*',b'.',b'.',b'.',b'.',b'.',b'.',b'*',b'0',b'0',b'0',b'*',b'.',b'.'],
        [b'*',b'*',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'*',b'0',b'0',b'0',b'*',b'.'],
        [b'*',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'*',b'0',b'0',b'0',b'*'],
        [b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'*',b'0',b'0',b'*'],
        [b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'*',b'*',b'*'],
    ];

        let p = cursor.as_ptr();

        unsafe {
            for y in 0..16 {
                for x in 0..16 {
                    if *((p as u32 + y * 16 + x) as *const u8) == b'*' {
                        *((mouse as u32 + y * 16 + x) as *mut u8)  = COL8_000000
                    }
                    if *((p as u32 + y * 16 + x) as *const u8) == b'0' {
                        *((mouse as u32 + y * 16 + x) as *mut u8)  = COL8_FFFFFF
                    }
                    if *((p as u32 + y * 16 + x) as *const u8) == b'.' {
                        *((mouse as u32 + y * 16 + x) as *mut u8)  = bc
                    }
                }
            }
        }
}

fn putblock8_8(vram: *const u32, vxsize: u32, pxsize: u32, pysize: u32, 
               px0: u32, py0: u32, buf: *mut u8, bxsize: u32){

    for y in 0..pysize {
        for x in 0..pxsize {
            unsafe {
                *((vram as u32 + (py0 + y) * vxsize + (px0 + x)) as *mut u32) =
                    *((buf as u32 + y * bxsize + x) as *mut u32)
            }
        }
    }
}

fn init_gdtidt(){
    let gdt = 0x00270000 as *mut (SEGMENT_DESCRIPTOR);
    let idt = 0x0026f800 as *mut (GATE_DESCRIPTOR);
        
    //setup GDT
    unsafe {
        for i in 0..8192 {
            set_segmdesc(gdt.offset(i), 0, 0, 0);
        }
        set_segmdesc(gdt.offset(1), 0xffffffff, 0x00000000, 0x4092);
        set_segmdesc(gdt.offset(2), 0x0007ffff, 0x00280000, 0x409a);
    }
	load_gdtr(0xffff, 0x00270000);

    //setup IDT
    for i in 0..256 {
        unsafe {
            set_gatedesc(idt.offset(i), 0, 0, 0);
        }
    }
	load_idtr(0x7ff, 0x0026f800);

}

fn set_segmdesc(sd: *mut SEGMENT_DESCRIPTOR, mut limit: u32, base: u32, mut ar: u32){

    if limit > 0xfffff {
        ar |= 0x8000; /* G_bit = 1 */
        limit /=  0x1000;
    }

    unsafe {
        (*sd).limit_low    = (limit & 0xffff) as u16;
        (*sd).base_low     = (base & 0xffff)  as u16;
        (*sd).base_mid     = ((base >> 16) as u8) & 0xff;
        (*sd).access_right = (ar & 0xff) as u8;
        (*sd).limit_high   = ((limit >> 16) & 0x0f) as u8 | ((ar >> 8) & 0xf0) as u8;
        (*sd).base_high    = ((base >> 24)  & 0xff) as u8;
    }
}

fn set_gatedesc(gd: *mut GATE_DESCRIPTOR, offset: u32, selector: u32, ar: u32){
    unsafe {
        (*gd).offset_low   = (offset & 0xffff) as u16;
        (*gd).selector     = selector as u16;
        (*gd).dw_count     = ((ar >> 8) & 0xff) as u8;
        (*gd).access_right = (ar & 0xff) as u8;
        (*gd).offset_high  = ((offset >> 16) & 0xffff) as u16;
    }
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

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
    let _edx: u32;
    let _al : u8;
    unsafe {
        asm!("movl $0,   %edx" :"={edx}"(_edx):"0"(port)::"volatile");
        asm!("movb $0,   %al"  :"={al}"(_al)  :"0"(data)::"volatile");
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
    let _eax: u32;
    unsafe {
        asm!("movl $0, %eax":"={eax}"(_eax):"0"(eflags)::"volatile");
        asm!("push %eax");
        asm!("popfd");
        asm!("ret");
    }
}

//_load_gdtr:		; void load_gdtr(int limit, int addr);
//		MOV		AX,[ESP+4]		; limit
//		MOV		[ESP+6],AX
//		LGDT	[ESP+6]
//		RET
#[no_mangle]
#[inline(never)]
pub extern fn load_gdtr(limit: u32, mut addr: u32){
    let ax: u16;
    let mut exp: u32;
    unsafe {
        asm!("movw $0, %ax":"={ax}"(ax):"0"(limit)::"volatile");
        asm!("movw %ax, $1":"=r"(addr):"{ax}"(ax),"r"(addr)::"volatile");
        asm!("lgdtw m32"::"m32"(addr)::);
        asm!("ret");
//        asm!("add $2, $0"
//             : "=r"(c)
//             : "0"(a), "r"(b)
//             );
    }
}

#[no_mangle]
#[inline(never)]
pub extern fn load_idtr(limit: u32, mut addr: u32){
    let ax: u16;
    let mut exp: u32;
    unsafe {
        asm!("movw $0, %ax":"={ax}"(ax):"0"(limit)::"volatile");
        asm!("movw %ax, $1":"=r"(addr):"{ax}"(ax),"r"(addr)::"volatile");
        exp = addr;
        asm!("lidtw idt"::"0"(addr)::"volatile");
        asm!("ret");
    }
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

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
