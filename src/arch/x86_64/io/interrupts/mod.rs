//!-----------------------------------------------------------------------------------------------
//!`src/io/interrupts/mod.rs`
//!
//!Structures for manipulating the IDT.
//!-----------------------------------------------------------------------------------------------

pub mod exceptions;

use io::cpuio::Port;

#[repr(C, packed)]
#[derive(Clone, Copy)]
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

	let mut idt: [IDTEntry; 256] = [IDTEntry::new(0, 0x08, 0x8e); 256];

	let kb_addr: usize;
	kb_addr = asm_kb_handler as *const () as usize;

	idt[0x21] = IDTEntry::new(kb_addr, 0x08, 0x8e);

	let init1 = Port::new(0x20);
	let init2 = Port::new(0xa0);
	let cfg1 = Port::new(0x21);
	let cfg2 = Port::new(0xa1);

	//Start config
	init1.outb(0x11);
	init2.outb(0x11);

	//Remap IRQs
	cfg1.outb(0x20);
	cfg2.outb(0x28);

	cfg1.outb(0x00);
	cfg2.outb(0x00);

	cfg1.outb(0x01);
	cfg2.outb(0x01);

	cfg1.outb(0xff);
	cfg2.outb(0xff);

	let idt_addr: u64;
	idt_addr = &idt as *const _ as u64;

	let idtr = IDTR::new((16 * 256) - 1, idt_addr);

	let idtr_addr: u64;
	idtr_addr = &idtr as *const _ as u64;

	print!("\nidtr_addr {:0x}", idtr_addr);

	unsafe {
		asm_lidt(idtr_addr);
		//Test it
		//asm_int_test();
	}

	//exceptions::divzero();

	//Enable keyboard
	cfg1.outb(0xfd);
	cfg2.outb(0xff);
}

//Assembly interrupt wrappers
extern {
	fn asm_lidt(idtr: u64);
	//fn asm_int_test();

	fn asm_kb_handler();
}
