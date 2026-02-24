pub fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}

pub fn outw(port: u16, value: u16) {
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack));
    }
}

pub fn outl(port: u16, value: u32) {
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
    }
}

pub fn inb(port: u16) -> u8 {
    unsafe {
        let res: u8;
        core::arch::asm!("in al, dx", in("dx") port, out("al") res, options(nomem, nostack));
        res
    }
}

pub fn inw(port: u16) -> u16 {
    unsafe {
        let res: u16;
        core::arch::asm!("in ax, dx", in("dx") port, out("ax") res, options(nomem, nostack));
        res
    }
}

pub fn inl(port: u16) -> u32 {
    unsafe {
        let res: u32;
        core::arch::asm!("in eax, dx", in("dx") port, out("eax") res, options(nomem, nostack));
        res
    }
}
