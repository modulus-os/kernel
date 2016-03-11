use io::cpuio::Port;
use io::display::*;
use core::fmt::Write;

//let mut tock = 0;

#[no_mangle]
pub extern fn kb_handler() {
	//EOI
	let data = Port::new(0x60);
	let scancode = data.inb();

	let eoi = Port::new(0x20);
	eoi.outb(0x20);

	let mut term = terminal::Terminal::new();

	write!(term, "Keyboard");
}
