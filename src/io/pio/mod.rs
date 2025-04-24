#![feature(asm_experimental_arch)]
use core::arch::asm;

#[inline(always)]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nostack, nomem, preserves_flags),
        );
    }
}

#[inline(always)]
pub fn outw(port: u16, value: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value,
            options(nostack, nomem, preserves_flags),
        );
    }
}

#[inline(always)]
pub fn outl(port: u16, value: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nostack, nomem, preserves_flags),
        );
    }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nostack, nomem, preserves_flags),
        );
    }
    value
}

#[inline(always)]
pub fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") value,
            options(nostack, nomem, preserves_flags),
        );
    }
    value
}

#[inline(always)]
pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") value,
            options(nostack, nomem, preserves_flags),
        );
    }
    value
}
