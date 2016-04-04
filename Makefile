ARCH = x86_64
TARGET = $(ARCH)-unknown-linux-gnu

OBJDUMP_FILE ?= target/modulon

BUILD_DIR = target

ASRC = $(wildcard src/asm/$(ARCH)/*.asm)
AOBJ = $(patsubst src/asm/$(ARCH)/%.asm, target/asm/$(ARCH)/%.o, $(ASRC))

ASM = nasm -f elf64
CARGO = cargo rustc --target $(TARGET) -- -Z no-landing-pads -C no-redzone
LD = ld --nmagic --gc-section -T src/arch/$(ARCH)/linker.ld

QEMU ?= -enable-kvm

run: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso $(QEMU)

all: target_dir target/modulon

travis:
	make all -j
	cargo test -j8

debug: target/modulon.iso
	qemu-system-x86_64 -cdrom target/modulon.iso -s -d int -no-reboot

objdump:
	touch objdump.txt
	objdump -D $(OBJDUMP_FILE) | cat >> objdump.txt

target/modulon: target_dir $(AOBJ)
	$(CARGO)
	$(LD) $(AOBJ) target/$(TARGET)/debug/libmodulon.a -o target/modulon

target/asm/$(ARCH)/%.o: src/asm/$(ARCH)/%.asm
	$(ASM) $< -o $@

target/modulon.iso: target/modulon src/arch/$(ARCH)/grub.cfg
	@mkdir -p target/iso/boot/grub
	@cp src/arch/$(ARCH)/grub.cfg target/iso/boot/grub
	@cp target/modulon target/iso/boot
	@echo
	@grub-mkrescue -o target/modulon.iso target/iso -d /usr/lib/grub/i386-pc

target_dir:
	@mkdir -p target/asm/$(ARCH)

clean:
	rm -rf target

doc-kernel:
	cargo doc
	rm -rf target/doc
	rustdoc src/lib.rs --crate-name modulon -o /home/voxl/kernel/target/doc -L \
	dependency=/home/voxl/kernel/target/debug -L dependency=/home/voxl/kernel/target/debug/deps \
	--extern bitflags=/home/voxl/kernel/target/debug/deps/libbitflags-b378ff20d60f43f8.rlib \
	--extern bitflags=/home/voxl/kernel/target/debug/deps/libbitflags-b378ff20d60f43f8.rlib \
	--extern multiboot2=/home/voxl/kernel/target/debug/deps/libmultiboot2-2476d95169c1d115.rlib \
	--extern multiboot2=/home/voxl/kernel/target/debug/deps/libmultiboot2-2476d95169c1d115.rlib \
	--extern spin=/home/voxl/kernel/target/debug/deps/libspin-c74e99cb6d9a92cb.rlib \
	--extern spin=/home/voxl/kernel/target/debug/deps/libspin-c74e99cb6d9a92cb.rlib
