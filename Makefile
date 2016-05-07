ARCH = x64
ARCH_FULL = x86_64
TARGET = $(ARCH_FULL)-unknown-linux-gnu

OBJDUMP_FILE ?= target/modulus

BUILD_DIR = target

ASRC = $(wildcard asm/$(ARCH)/*.asm)
AOBJ = $(patsubst asm/$(ARCH)/%.asm, target/asm/$(ARCH)/%.o, $(ASRC))

ASM = nasm -f elf64
CARGO = cargo rustc --target $(TARGET) -- -Z no-landing-pads -C no-redzone
LD = ld --nmagic --gc-section -T src/$(ARCH)/linker.ld

bochs: target/modulus.iso
	bochs -f bochs.x86_64 -q

qemu: target/modulus.iso
	qemu-system-x86_64 -hda target/modulus.iso -s -d int -no-reboot $(QEMU)

all: target_dir target/modulus

travis:
	make all -j
	cargo test -j8

objdump:
	touch objdump.txt
	objdump -D $(OBJDUMP_FILE) | cat >> objdump.txt

target/modulus: target_dir $(AOBJ)
	$(CARGO)
	$(LD) $(AOBJ) target/$(TARGET)/debug/libmodulus.a -o target/modulus

target/asm/$(ARCH)/%.o: asm/$(ARCH)/%.asm
	$(ASM) $< -o $@

target/modulus.iso: target/modulus src/$(ARCH)/grub.cfg
	@mkdir -p target/iso/boot/grub
	@cp src/$(ARCH)/grub.cfg target/iso/boot/grub
	@cp target/modulus target/iso/boot
	@cp -r fs/* target/iso
	@grub-mkrescue -o target/modulus.iso target/iso > /dev/null 2>&1
	@#dd if=/dev/zero of=disk.img bs=512 count=131071
	@#mkfs.vfat -F 32 -I disk.img

target_dir:
	@mkdir -p target/asm/$(ARCH)

clean:
	rm -rf target
	cargo fmt -- src/lib.rs --write-mode=overwrite

doc-kernel:
	cargo doc
	rm -rf target/doc
	rustdoc src/lib.rs --crate-name modulus -o /home/voxl/kernel/target/doc -L \
	dependency=/home/voxl/kernel/target/debug -L dependency=/home/voxl/kernel/target/debug/deps \
	--extern bitflags=/home/voxl/kernel/target/debug/deps/libbitflags-b378ff20d60f43f8.rlib \
	--extern bitflags=/home/voxl/kernel/target/debug/deps/libbitflags-b378ff20d60f43f8.rlib \
	--extern multiboot2=/home/voxl/kernel/target/debug/deps/libmultiboot2-2476d95169c1d115.rlib \
	--extern multiboot2=/home/voxl/kernel/target/debug/deps/libmultiboot2-2476d95169c1d115.rlib \
	--extern spin=/home/voxl/kernel/target/debug/deps/libspin-c74e99cb6d9a92cb.rlib \
	--extern spin=/home/voxl/kernel/target/debug/deps/libspin-c74e99cb6d9a92cb.rlib
