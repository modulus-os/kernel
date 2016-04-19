/// 64-bit IDT descriptor entry
pub struct Entry {
    offset_low: u16,
    selector: u16,
    zero1: u8,
    typ_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero2: u32,
}

impl Entry {
	pub fn new(address: u64, selector: u16, typ_attr: u8) -> Self {
		Entry {
			offset_low: (address & 0x000000000000ffff) as u16,
			selector: selector,
			zero1: 0,
			typ_attr: typ_attr,
			offset_mid: ((address & 0x00000000ffff0000) >> 16) as u16,
			offset_high: ((address & 0xffffffff00000000) >> 32) as u32,
			zero2: 0,
		}
	}
}
