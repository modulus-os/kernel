use io::pio::*;

const PIC_1C: u16 = 0x20;
const PIC_2C: u16 = 0xa0;
const PIC_1D: u16 = 0x21;
const PIC_2D: u16 = 0xa1;

pub fn remap(offset1: u8, offset2: u8) {
    // Save current masks
    let mask1 = inb(PIC_1D);
    let mask2 = inb(PIC_2D);

    // Start init
    outb(PIC_1C, 0x11);
    outb(PIC_2C, 0x11);

    // Vector offset
    outb(PIC_1D, offset1);
    outb(PIC_2D, offset2);

    // Master/slave wiring
    outb(PIC_1D, 4);
    outb(PIC_2D, 2);

    // Additional information
    outb(PIC_1D, 1);
    outb(PIC_2D, 1);

    // Restore masks
    outb(PIC_1D, mask1);
    outb(PIC_2D, mask2);
}

pub fn mask(irq: u8) {
    if irq < 0x8 {
        let masks = inb(PIC_1D);
        outb(PIC_1D, masks | 1 << irq);
    }

    if irq >= 0x8 && irq < 0x10 {
        let masks = inb(PIC_2D);
        outb(PIC_2D, masks | 1 << (irq - 0x8));
    }
}

pub fn unmask(irq: u8) {
    if irq < 0x8 {
        let masks = inb(PIC_1D);
        outb(PIC_1D, masks & !(1 << irq));
    }

    if irq >= 0x8 && irq < 0x10 {
        let masks = inb(PIC_2D);
        outb(PIC_2D, masks & !(1 << (irq - 0x8)));
    }
}
