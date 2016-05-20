top = "."
out = "target"

def configure(cfg):
	cfg.find_program("nasm", VAR="NASM")
	cfg.find_program("cargo", VAR="CARGO")
	cfg.find_program("ld", VAR="LD")
	cfg.find_program("grub-mkrescue", VAR="GRUBR")

	cfg.env.TARGET = "x64"
	cfg.env.TRIPLE = "x86_64-unknown-linux-gnu"

	cfg.env.NASM_FLAGS = "-f elf64 -i../asm/x64/"
	cfg.env.ASM_MAIN = "../asm/x64/boot.asm"

	cfg.env.CARGO_FLAGS = "--target x86_64-unknown-linux-gnu -- -Z no-landing-pads -C no-redzone"

	cfg.env.LD_FLAGS = "--nmagic --gc-section -T ../scripts/linker.ld"

def options(opt):
	opt.add_option("-q", "--qemu", dest="qemu", default=False, action="store_true", help="Run in QEMU")
	opt.add_option("-b", "--bochs", dest="bochs", default=False, action="store_true", help="Run in Bochs")
	opt.add_option("--no-iso", dest="build_iso", default=True, action="store_false", help="Do not build ISO image")

def build(bld):
	# Rust
	bld(rule="${CARGO} rustc ${CARGO_FLAGS}",
		source=bld.path.ant_glob("src/**"),
		target="../target/x86_64-unknown-linux-gnu/debug/libmodulus.a", shell=True)

	# Assembly
	bld(rule="${NASM} ${NASM_FLAGS} ${ASM_MAIN} -o asm.o",
		source=bld.path.ant_glob("asm/${TARGET}/*.asm"),
		target="asm.o")

	# Link
	bld(rule="${LD} ${LD_FLAGS} asm.o ../target/x86_64-unknown-linux-gnu/debug/libmodulus.a -o modulus",
		target="modulus",
		source="../target/x86_64-unknown-linux-gnu/debug/libmodulus.a")

	# ISO image
	if (bld.options.build_iso):
		bld(rule="sh ../scripts/iso.sh",
			target="modulus.iso",
			source="modulus")

	if (bld.options.qemu and bld.options.bochs):
		print("Error: Both QEMU and Bochs selected")
	elif (bld.options.qemu):
		bld(rule="qemu-system-x86_64 -hda modulus.iso -d int", always=True, source="modulus.iso")
	elif (bld.options.bochs):
		bld(rule="bochs -f ../bochs.x86_64 -q", always=True, source="modulus.iso")
