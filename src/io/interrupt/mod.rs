//!-----------------------------------------------------------------------------------------------
//!`src/io/interrupt/mod.rs`
//!
//!Structures for manipulating the IDT.
//!-----------------------------------------------------------------------------------------------

pub mod keyboard;

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

pub fn init_idt() {
	let kb_addr = asm_kb_handler as *const usize;
	//let kb_entry = Entry::new();
}

//Assembly interrupt wrappers
extern {
	fn asm_kb_handler();
}
