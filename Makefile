ARCH = x86_64

MODULES = . display display/terminal memory lib cpuio
RSRC_DIR = $(addprefix src/, $(MODULES))
BUILD_DIR = $(addprefix build/, $(MODULES))

RSRC = $(foreach sdir, $(RSRC_DIR), $(wildcard $(sdir)/*.rs))
ASRC = $(wildcard arch/$(ARCH)/*.asm)

ROBJ = $(patsubst src/%.rs, build/%.o, $(RSRC))
AOBJ = $(patsubst arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(ASRC))

ASM = nasm -f elf64
RUSTC = rustc -Z no-landing-pads -C no-redzone
LD = ld -n --gc-sections -T arch/$(ARCH)/linker.ld

all: $(BUILD_DIR) build/modulon

run: build/modulon.iso
	qemu-system-x86_64 -cdrom build/modulon.iso -s

debug: build/modulon.iso
	qemu-system-x86_64 -cdrom build/modulon.iso -s -S

objdump: build/modulon
	touch objdump.txt
	objdump -D build/modulon | cat >> objdump.txt

build/modulon: $(BUILD_DIR) $(AOBJ) build/main.o
	$(LD) $(AOBJ) build/main.o -o build/modulon

build/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	$(ASM) $< -o $@

build/main.o: $(RSRC)
	$(RUSTC) src/main.rs -o $@ --crate-type staticlib

build/modulon.iso: build/modulon arch/$(ARCH)/grub.cfg
	@mkdir -p build/iso/boot/grub
	@cp arch/$(ARCH)/grub.cfg build/iso/boot/grub
	@cp build/modulon build/iso/boot
	@echo
	@grub-mkrescue -o build/modulon.iso build/iso -d /usr/lib/grub/i386-pc

$(BUILD_DIR):
	@mkdir -p $@
	@mkdir -p build/arch/$(ARCH)

clean:
	rm -rf build
