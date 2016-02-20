ARCH = x86_64

MODULES = . display display/terminal rlibc
RSRC_DIR = $(addprefix src/, $(MODULES))
BUILD_DIR = $(addprefix build/, $(MODULES))

RSRC = $(foreach sdir, $(RSRC_DIR), $(wildcard $(sdir)/*.rs))
ASRC = $(wildcard arch/$(ARCH)/*.asm)

ROBJ = $(patsubst src/%.rs, build/%.o, $(RSRC))
AOBJ = $(patsubst arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(ASRC))

ASM = nasm -f elf64
RUSTC = rustc -Z no-landing-pads -C no-redzone
LD = ld -n --gc-sections -T arch/$(ARCH)/linker.ld

run: all build/modulon.iso
	qemu-system-x86_64 -cdrom build/modulon.iso 

all: $(BUILD_DIR) build/modulon

build/modulon: $(AOBJ) build/main.o
	$(LD) $(AOBJ) build/main.o -o build/modulon

build/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	$(ASM) $< -o $@

build/main.o: $(RSRC)
	$(RUSTC) src/main.rs -o $@ --crate-type staticlib

build/modulon.iso: build/modulon arch/$(ARCH)/efi.img arch/$(ARCH)/grub.cfg
	@mkdir -p build/iso/boot/grub
	@cp arch/$(ARCH)/grub.cfg build/iso/boot/grub
	@cp arch/$(ARCH)/efi.img build/iso
	@cp build/modulon build/iso/boot
	@echo
	@grub-mkrescue -o build/modulon.iso build/iso

$(BUILD_DIR):
	@mkdir -p $@
	@mkdir -p build/arch/$(ARCH)

clean:
	rm -rf build
