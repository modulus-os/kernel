/// PIC functions
pub mod pic;

/// 64-bit IDT descriptor entry
#[derive(Copy, Clone, Debug)]
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

pub struct Idtr {
    limit: u16,
    address: u64,
}

pub struct Idt {
    entries: [Entry; 256],
}

impl Idt {
    pub fn new() -> Self {
        Idt { entries: [Entry::new(0x0, 0x0, 0x0); 256] }
    }

    pub fn add_isr(&mut self, index: usize, isr: Entry) {
        self.entries[index] = isr;
    }

    pub fn install(&mut self) {
        let limit = 16 * 256 - 1;
        let address = &self.entries[0] as *const _ as u64;

        let divzero = &asm_divzero as *const _ as u64;
        self.add_isr(0x0, Entry::new(divzero, 0x8, 0x8e));

        let idtr = Idtr {
            limit: limit,
            address: address,
        };
    }
}

extern "C" {
    fn asm_divzero();

    fn asm_lidt(idtr: Idtr);
}
