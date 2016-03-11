//!-----------------------------------------------------------------------------------------------
//!`src/io/interrupt/mod.rs`
//!
//!Structures for manipulating the IDT.
//!-----------------------------------------------------------------------------------------------

pub mod keyboard;

use io::cpuio::Port;
use io::display::*;
use core::fmt::Write;

#[repr(C, packed)]
pub struct IDTEntry {
	offset_low: u16,
	selector: u16,
	zero1: u8,
	type_attr: u8,
	offset_mid: u16,
	offset_high: u32,
	zero2: u32,
}

impl IDTEntry {
	pub fn new(offset: usize, selector: u16, type_attr: u8) -> IDTEntry{
		IDTEntry {
			offset_low: (offset & 0xffff) as u16,
			selector: selector,
			zero1: 0,
			type_attr: type_attr,
			offset_mid: ((offset & 0xffff0000) >> 16) as u16,
			offset_high: ((offset & 0xffffffff00000000) >> 32) as u32,
			zero2: 0
		}
	}
}

#[repr(C, packed)]
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
	let mut term = terminal::Terminal::new();

	let kb_addr: usize;
	unsafe {
		kb_addr = asm_kb_handler as *const () as usize;
	}

	let idt = IDTEntry::new(kb_addr, 0x8, 0x8e);

	let init1 = Port::new(0x20);
	let init2 = Port::new(0xA0);
	let cfg1 = Port::new(0x21);
	let cfg2 = Port::new(0xA1);

	//Start config
	init1.outb(0x11);
	init2.outb(0x11);

	cfg1.outb(0x20);
	cfg2.outb(0x28);

	cfg1.outb(0x00);
	cfg2.outb(0x00);

	cfg1.outb(0x01);
	cfg2.outb(0x01);

	cfg1.outb(0xff);
	cfg2.outb(0xff);

	let idt_addr: usize;
	unsafe {idt_addr = &idt as *const _ as usize;}
	let idtr = IDTR::new(12 as u16, idt_addr as u64);

	term.newline();
	term.newline();
	write!(term, "idt_addr {}", idt_addr);

	unsafe {
		asm_lidt(&idtr as *const _ as u64);
	}

	cfg1.outb(0xfd);
}

//Assembly interrupt wrappers
extern {
	fn asm_lidt(idtr: u64);
	fn asm_kb_handler();
}
