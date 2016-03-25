ARCH = x86_64
TARGET = $(ARCH)-unknown-linux-gnu

OBJDUMP_FILE ?= target/modulon

BUILD_DIR = target

ASRC = $(wildcard arch/$(ARCH)/*.asm)
AOBJ = $(patsubst arch/$(ARCH)/%.asm, target/arch/$(ARCH)/%.o, $(ASRC))

ASM = nasm -f elf64
CARGO = cargo rustc --target $(TARGET) -- -Z no-landing-pads -C no-redzone
LD = ld --nmagic --gc-section -T arch/$(ARCH)/cfg/linker.ld

all: target_dir target/modulon

run: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso

debug: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso -s -d int -no-reboot

objdump:
	touch objdump.txt
	objdump -D $(OBJDUMP_FILE) | cat >> objdump.txt

target/modulon: target_dir $(AOBJ)
	$(CARGO)
	$(LD) $(AOBJ) target/$(TARGET)/debug/libkmain.a -o target/modulon

target/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	$(ASM) $< -o $@

target/modulon.iso: target/modulon arch/$(ARCH)/cfg/grub.cfg
	@mkdir -p target/iso/boot/grub
	@cp arch/$(ARCH)/cfg/grub.cfg target/iso/boot/grub
	@cp target/modulon target/iso/boot
	@echo
	@grub-mkrescue -o target/modulon.iso target/iso -d /usr/lib/grub/i386-pc

target_dir:
	@mkdir -p target/arch/$(ARCH)

clean:
	rm -rf target
