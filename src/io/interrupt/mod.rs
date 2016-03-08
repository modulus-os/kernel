//!-----------------------------------------------------------------------------------------------
//!`src/io/interrupt/mod.rs`
//!
//!Structures for manipulating the IDT.
//!-----------------------------------------------------------------------------------------------

pub mod keyboard;

use io::cpuio::Port;

pub struct Entry {
	offset_low: u16,
	selector: u16,
	zero1: u8,
	type_attr: u8,
	offset_mid: u16,
	offset_high: u32,
	zero2: u32,
}

impl Entry {
	pub fn new(offset_low: u16, selector: u16, type_attr: u8,
		offset_mid: u16, offset_high: u32) -> Entry{
		Entry {
			offset_low: offset_low,
			selector: selector,
			zero1: 0,
			type_attr: type_attr,
			offset_mid: offset_mid,
			offset_high: offset_high,
			zero2: 0
		}
	}
}

pub struct IDTR {
	limit: u16,
	offset: u64,
}

impl IDTR {
	pub fn new(limit: u16, offset: u64) -> IDTR {
		IDTR {limit: limit, offset: offset}
	}
}

pub fn init_idt() {
	let kb_addr: *const ();
	unsafe {
		kb_addr = asm_kb_handler as *const ();
	}

	let mut idt: [Entry; 1] = [
		Entry::new(kb_addr as u16 & 0xffff, 0x08, 0x8e,
			(kb_addr as u16 & 0xffff0000) >> 8,
			(kb_addr as u32 & 0xffffffff00000000) >> 16)
	];

	let icw1_1 = Port::new(0x20);
	let icw1_2 = Port::new(0xA0);
	let icw234_1 = Port::new(0x21);
	let icw234_2 = Port::new(0xA1);

	//Start config
	icw1_1.outb(0x11);
	icw1_2.outb(0x11);

	//
	icw234_1.outb(0x20);
	icw234_2.outb(0x28);

	icw234_1.outb(0x00);
	icw234_2.outb(0x00);

	icw234_1.outb(0x01);
	icw234_2.outb(0x01);

	icw234_1.outb(0xff);
	icw234_2.outb(0xff);

	//unsafe {idt_addr = *(idt[0] as *const Entry);}
	//let idtr = IDTR::new((12 * idt.len()) as u16, idt_addr as u64);

	lidt();
}

fn lidt() {
}

//Assembly interrupt wrappers
extern {
	fn asm_kb_handler();
}
