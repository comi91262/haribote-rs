TOOLPATH = ../z_tools/
INCPATH  = ../z_tools/haribote/

MAKE     = make -r
LD       = $(HOME)/local/i386-elf/bin/ld
NASK     = $(TOOLPATH)nask
CC1      = $(TOOLPATH)gocc1 -I$(INCPATH) -Os -Wall -quiet
EDIMG    = $(TOOLPATH)edimg
QEMU		= qemu-system-i386
COPY     = cp
DEL      = rm -f

default :
	$(MAKE) img

#¿¿¿¿¿¿

ipl10.bin : ipl10.nas Makefile
	$(NASK) ipl10.nas ipl10.bin ipl10.lst

asmhead.bin : asmhead.nas Makefile
	$(NASK) asmhead.nas asmhead.bin asmhead.lst

body.bin: 
	$(LD) -n --gc-sections -e rust_main -Ttext 0x280000 -o body.bin libbody.a

#   $(HOME)/local/i386-elf/bin/ld -n --gc-sections -e rust_main -T linker.ld -o body.bin body-e6e43481cde3b4d7.o multiboot_header.o
#	ld -n --gc-sections -T linker.ld -o body.bin body-06ded3dca6281142.o multiboot_header.asm 


haribote.sys : asmhead.bin body.bin Makefile
	cat asmhead.bin body.bin > haribote.sys

haribote.img : ipl10.bin haribote.sys Makefile
	$(EDIMG)   imgin:$(TOOLPATH)fdimg0at.tek \
		wbinimg src:ipl10.bin len:512 from:0 to:0 \
		copy from:haribote.sys to:@: \
		imgout:haribote.img

img :
	$(MAKE) haribote.img

run :
	$(MAKE) img
	$(QEMU) -L . -m 32 -localtime -vga std -fda $(TOOLPATH)qemu/fdimage0.bin


clean :
	-$(DEL) *.bin
	-$(DEL) *.lst
	-$(DEL) haribote.sys
	-$(DEL) rust_os.img
	-$(DEL) haribote_rs.sys


src_only :
	$(MAKE) clean
	-$(DEL) haribote.img
