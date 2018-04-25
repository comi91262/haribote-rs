//
////_io_out8:	; void io_out8(int port, int data);
////		MOV		EDX,[ESP+4]		; port
////		MOV		AL,[ESP+8]		; data
////		OUT		DX,AL
////		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_out8(port: u32, data: u8){
//    let _edx: u32;
//    let _al : u8;
//    unsafe {
//      //  asm!("mov 4(%esp), %edx");
//      //  asm!("mov 8(%esp), %al");
//      //  asm!("out %al, %dx");
//        asm!("movl $0,   %edx" :"={edx}"(_edx):"0"(port)::"volatile");
//        asm!("movb $0,   %al"  :"={al}"(_al)  :"0"(data)::"volatile");
//        asm!("out  %al,  %dx");
//
//    }
//}
//
////_io_load_eflags:	; int io_load_eflags(void);
////		PUSHFD		; PUSH EFLAGS Æ¢¤Ó¡
////		POP		EAX
////		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_load_eflags() -> u32 {
//    let eax;
//    unsafe {
//        asm!("pushfd");
//        asm!("pop %eax":"={eax}"(eax));
//    }
//    eax
//}
//
////_io_store_eflags:	; void io_store_eflags(int eflags);
////		MOV		EAX,[ESP+4]
////		PUSH	EAX
////		POPFD		; POP EFLAGS Æ¢¤Ó¡
////		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_store_eflags(eflags: u32){
//    let _eax: u32;
//    unsafe {
//        asm!("movl $0, %eax":"={eax}"(_eax):"0"(eflags)::"volatile");
//        asm!("push %eax");
//        asm!("popfd");
//        asm!("ret");
//    }
//}
//
////_load_gdtr:		; void load_gdtr(int limit, int addr);
////		MOV		AX,[ESP+4]		; limit
////		MOV		[ESP+6],AX
////		LGDT	[ESP+6]
////		RET
//#[no_mangle]
//#[inline(never)]
//#[cfg(any(target_arch = "x86"))]
//pub extern fn load_gdtr(limit: u32, mut addr: u32){
//    let ax: u16;
//    unsafe {
//        asm!("mov 4(%esp), %ax");
//        asm!("mov %ax, 6(%esp)");
//        asm!("lgdt 6(%esp)");
//        asm!("ret");
//        //asm!("mov $0, %ax"::"0"(limit as u16):"ax":);
//        //asm!("movw %ax, $1":"=r"(addr as u16):"{ax}"(ax),"r"(addr as u16):"ax":"volatile");
//        //asm!("movw %ax, $1":"=r"(addr):"{ax}"(ax),"r"(addr)::"volatile");
//        //asm!("lgdtw m32"::"m32"(addr)::);
//        //asm!("ret");
////        asm!("add $2, $0"
////             : "=r"(c)
////             : "0"(a), "r"(b)
////             );
//    }
//}
//
//#[no_mangle]
//#[inline(never)]
//pub extern fn load_idtr(limit: u32, mut addr: u32){
//    let ax: u16;
//    let mut exp: u32;
//    unsafe {
//        asm!("mov 4(%esp), %ax");
//        asm!("mov %ax, 6(%esp)");
//        asm!("lidt 6(%esp)");
//        asm!("ret");
//        //asm!("movw $0, %ax":"={ax}"(ax):"0"(limit)::"volatile");
//        //asm!("movw %ax, $1":"=r"(addr):"{ax}"(ax),"r"(addr)::"volatile");
//        //exp = addr;
//        //asm!("lidtw idt"::"0"(addr)::"volatile");
//        //asm!("ret");
//    }
//}
//
//
////_io_in8:	; int io_in8(int port);
////		MOV		EDX,[ESP+4]		; port
////		MOV		EAX,0
////		IN		AL,DX
////		RET
////#[no_mangle]
////#[inline(never)]
////pub extern fn io_in8(port: u32){
////    let esp: u32;
////    unsafe {
////        asm!("movl $0,  4(%esp)" :"={esp}"(esp):"0"(port)::"volatile");
////        asm!("movl $$0, %eax"    ::::"volatile");
////        asm!("in %al, %dx");
////        asm!(ret);
////    }
////}
////#[no_mangle]
////#[inline(never)]
////pub extern pub extern fn write_mem8(addr: u32, data: u32){
////    unsafe {
////        let ecx: u32;
////        let al:   u8;
////        asm!("movl $0,   %ecx" :"={ecx}"(ecx):"0"(addr)::"volatile");
////        asm!("movb $0,   %al"  :"={al}"(al):"0"(data)::"volatile");
////        asm!("movb %al, (%ecx)"::"%al"(al)::"volatile");
////    }
////}
////_io_cli:	; void io_cli(void);
////		CLI
////		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_cli(){
//    unsafe {
//        asm!("cli");
//        asm!("ret");
//    }
//}
//
////_io_sti:	; void io_sti(void);
////		STI
////		RET
//#[no_mangle]
//#[inline(never)]
//pub extern fn io_sti(){
//    unsafe {
//        asm!("sti");
//        asm!("ret");
//    }
//}
//
//
//
