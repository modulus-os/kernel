/// Defines exception handlers
pub mod exceptions;

/// Defines ISRs
pub mod isrs;

/// PIC functions
pub mod pic;

pub fn init() {
    let mut idt: [Entry; 256] = [Entry::new(0x0, 0x0, 0x0); 256];

    idt[0] = Entry::new(exceptions::de as *const () as u64, 0x8, 0x8e);
    idt[1] = Entry::new(exceptions::db as *const () as u64, 0x8, 0x8e);
    idt[2] = Entry::new(exceptions::nmi as *const () as u64, 0x8, 0x8e);
    idt[3] = Entry::new(exceptions::bp as *const () as u64, 0x8, 0x8e);
    idt[4] = Entry::new(exceptions::of as *const () as u64, 0x8, 0x8e);
    idt[5] = Entry::new(exceptions::br as *const () as u64, 0x8, 0x8e);
    idt[6] = Entry::new(exceptions::ud as *const () as u64, 0x8, 0x8e);
    idt[7] = Entry::new(exceptions::nm as *const () as u64, 0x8, 0x8e);
    idt[8] = Entry::new(exceptions::df as *const () as u64, 0x8, 0x8e);
    idt[9] = Entry::new(exceptions::cmf as *const () as u64, 0x8, 0x8e);
    idt[10] = Entry::new(exceptions::ts as *const () as u64, 0x8, 0x8e);
    idt[11] = Entry::new(exceptions::np as *const () as u64, 0x8, 0x8e);
    idt[12] = Entry::new(exceptions::ss as *const () as u64, 0x8, 0x8e);
    idt[13] = Entry::new(exceptions::gp as *const () as u64, 0x8, 0x8e);
    idt[14] = Entry::new(exceptions::pf as *const () as u64, 0x8, 0x8e);
    // Interrupt 15 is reserved
    idt[16] = Entry::new(exceptions::mf as *const () as u64, 0x8, 0x8e);
    idt[17] = Entry::new(exceptions::ac as *const () as u64, 0x8, 0x8e);
    idt[18] = Entry::new(exceptions::mc as *const () as u64, 0x8, 0x8e);
    idt[19] = Entry::new(exceptions::xm as *const () as u64, 0x8, 0x8e);
    idt[20] = Entry::new(exceptions::ve as *const () as u64, 0x8, 0x8e);

    idt[0x21] = Entry::new(asm_kb as *const () as u64, 0x8, 0x8e);

    idt[0x80] = Entry::new(asm_sys as *const () as u64, 0x8, 0x8e);

    let address = &idt[0] as *const _ as u64;

    unsafe {
        asm_lidt(address);
    }
}

/// 64-bit IDT descriptor entry
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
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
    pub const fn new(address: u64, selector: u16, typ_attr: u8) -> Self {
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


extern "C" {
    fn asm_sys();
    fn asm_kb();
    fn asm_lidt(idtr: u64);
}
