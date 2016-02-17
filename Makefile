bd = build
bdc = mkdir -p $(bd)

arch = x86_64
asrc = arch/$(arch)

run: modulon.iso
	qemu-system-x86_64 -cdrom modulon.iso

modulon-$(arch): $(bd)/mb_header.o $(bd)/boot.o
	$(bdc)
	ld -n -T $(asrc)/linker.ld -o modulon-$(arch) $(bd)/mb_header.o $(bd)/boot.o

$(bd)/mb_header.o: $(asrc)/mb_header.asm
	$(bdc)
	nasm -f elf64 $(asrc)/mb_header.asm -o $(bd)/mb_header.o

$(bd)/boot.o: $(asrc)/boot.asm
	$(bdc)
	nasm -f elf64 $(asrc)/boot.asm -o $(bd)/boot.o

modulon.iso: modulon-$(arch)
	$(bdc)
	mkdir -p $(bd)/grub/boot/grub
	cp $(asrc)/grub.cfg $(bd)/grub/boot/grub
	cp $(asrc)/efi.img $(bd)/grub
	cp modulon-$(arch) $(bd)/grub/boot
	grub-mkrescue -o modulon.iso $(bd)/grub

clean: $(bd)
	rm -rf $(bd)
