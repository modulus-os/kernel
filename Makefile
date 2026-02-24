LD        := x86_64-elf-ld
MKRESCUE  := i686-elf-grub-mkrescue
TARGET    := x86_64-unknown-none
KERNEL    := target/$(TARGET)/debug/libmodulus.a
ASM_OBJ   := target/asm.o
BINARY    := target/modulus
ISO       := target/modulus.iso
ISO_DIR   := target/iso

.PHONY: build iso run clean

build: $(BINARY)

$(KERNEL): $(shell find src -name '*.rs') Cargo.toml
	cargo build --target $(TARGET)

$(ASM_OBJ): asm/x64/boot.asm $(wildcard asm/x64/*.inc)
	nasm -f elf64 -iasm/x64/ $< -o $@

$(BINARY): $(ASM_OBJ) $(KERNEL)
	$(LD) --nmagic --gc-sections -T scripts/linker.ld $^ -o $@

iso: $(ISO)

$(ISO): $(BINARY) scripts/grub.cfg $(wildcard fs/*)
	mkdir -p $(ISO_DIR)/boot/grub
	cp scripts/grub.cfg $(ISO_DIR)/boot/grub/
	cp $(BINARY) $(ISO_DIR)/boot/
	cp -r fs/* $(ISO_DIR)/ 2>/dev/null || true
	$(MKRESCUE) -o $@ $(ISO_DIR)

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO)

clean:
	cargo clean
	rm -rf $(ASM_OBJ) $(BINARY) $(ISO) $(ISO_DIR)
