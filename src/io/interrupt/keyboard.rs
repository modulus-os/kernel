use io::cpuio::Port;

//let mut tock = 0;

#[no_mangle]
pub extern fn kb_handler() {
	//EOI
	let data = Port::new(0x60);
	let scancode = data.inb();

	let eoi = Port::new(0x20);
	eoi.outb(0x20);

	//let mut term = io::display::terminal::Terminal::new();

	//write!(term, "INdT");
}
