BD = build

ARCH = x86_64
ASRC = src/arch/$(ARCH)

KERNEL = modulon64

LD = ld -n -gc-sections -T $(ASRC)/linker.ld
ASM = nasm -f elf64

ASM_SRC = $(wildcard $(ASRC)/*.asm)

RUSTO = target/debug/libmodulon.a
ASMO = $(patsubst $(ASRC)/%.asm, $(BD)/arch/$(ARCH)/%.asm.o, $(ASM_SRC))

run: $(KERNEL).iso
	qemu-system-x86_64 -cdrom modulon.iso

$(KERNEL): $(ASMO) cargo $(RUSTO)
	@mkdir -p $(BD)/arch/$(ARCH)
	$(LD) -o $(KERNEL) $(ASMO) $(RUSTO)

$(BD)/arch/$(ARCH)/%.asm.o: $(ASRC)/%.asm
	$(ASM) $< -o $@

cargo:
	@cargo rustc  -- -Z no-landing-pads -C no-redzone

$(KERNEL).iso: $(KERNEL) $(ASRC)/grub.cfg $(ASRC)/efi.img
	@mkdir -p $(BD)/grub/boot/grub
	@cp $(ASRC)/grub.cfg $(BD)/grub/boot/grub
	@cp $(ASRC)/efi.img $(BD)/grub
	@cp modulon64 $(BD)/grub/boot
	@echo
	grub-mkrescue -o modulon.iso $(BD)/grub

clean: $(BD)
	rm -rf $(BD)
