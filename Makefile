bd = build
bdc = mkdir -p $(bd)

arch ?= x86_64
asrc = arch/$(arch)

kernel = modulon64

ld = $(asrc)/linker.ld
asmo := $(wildcard $(bd)/*.o)


run: modulon.iso
	qemu-system-x86_64 -cdrom modulon.iso -curses

modulon64: $(asmo)
	$(bdc)
	nasm $(asrc)/boot.asm -o $(bd)/boot.o -f elf64
	nasm $(asrc)/mb_header.asm -o $(bd)/mb_header.o -f elf64
	nasm $(asrc)/lm_start.asm -o $(bd)/lm_start.o -f elf64
	ld -n -T $(asrc)/linker.ld -o modulon64 $(bd)/boot.o $(bd)/mb_header.o $(bd)/lm_start.o

modulon.iso: modulon64
	mkdir -p $(bd)/grub/boot/grub
	cp $(asrc)/grub.cfg $(bd)/grub/boot/grub
	cp $(asrc)/efi.img $(bd)/grub
	cp modulon64 $(bd)/grub/boot
	grub-mkrescue -o modulon.iso $(bd)/grub

clean: $(bd)
	rm -rf $(bd)
