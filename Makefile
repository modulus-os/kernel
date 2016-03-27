ARCH = x86_64
TARGET = $(ARCH)-unknown-linux-gnu

OBJDUMP_FILE ?= target/modulon

BUILD_DIR = target

ASRC = $(wildcard src/asm/*.asm)
AOBJ = $(patsubst src/asm/%.asm, target/asm/%.o, $(ASRC))

ASM = nasm -f elf64
CARGO = cargo rustc --target $(TARGET) -- -Z no-landing-pads -C no-redzone
LD = ld --nmagic --gc-section -T src/arch/$(ARCH)/linker.ld

all: target_dir target/modulon

travis:
	make all
	cargo test

run: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso

debug: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso -s -d int -no-reboot

objdump:
	touch objdump.txt
	objdump -D $(OBJDUMP_FILE) | cat >> objdump.txt

target/modulon: target_dir $(AOBJ)
	$(CARGO)
	$(LD) $(AOBJ) target/$(TARGET)/debug/libmodulon.a -o target/modulon

target/asm/%.o: src/asm/%.asm
	$(ASM) $< -o $@

target/modulon.iso: target/modulon src/grub.cfg
	@mkdir -p target/iso/boot/grub
	@cp src/grub.cfg target/iso/boot/grub
	@cp target/modulon target/iso/boot
	@echo
	@grub-mkrescue -o target/modulon.iso target/iso -d /usr/lib/grub/i386-pc

target_dir:
	@mkdir -p target/asm

clean:
	rm -rf target
