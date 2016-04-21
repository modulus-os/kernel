#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![no_std]

// Multiboot crate for retrieving boot information
extern crate multiboot2;

// Spinlock crate
extern crate spin;

#[macro_use]
/// Macros
pub mod macros;
/// x86_64 specific code
///
/// This module contains code specific to the x86_64 architecture such as MM, interrupts, etc.
pub mod x86_64;
/// Common functions
///
/// Contains common kernel functions such as power management and panic handling.
pub mod common;
/// Standard support library
///
/// Contains memory manipulation functions such as memcpy, memmove, memset, and memcmp.
/// Rust depends on these functions to compile.
pub mod support;
/// Architecture independent I/O drivers
///
/// Includes Port I/O and display drivers.
pub mod io;

// Version information
pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MID: u16 = 1;
pub const VERSION_MINOR: u16 = 7;
pub const VERSION_COMMIT: u16 = 1;

// Reexport x86_64 architecture components
pub use x86_64::*;

use io::display::*;

/// Kernel main
///
/// This is the main kernel entry point. It is called by `src/asm/x86_64/lm_start.asm`.
/// This function initializes the kernel.
#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {

    // Clear text-mode terminal
    terminal::TERM.lock().clear();

    // Display version information
    terminal::TERM.lock().set_color(common_color::GREEN);
    print!("Modulus");
    terminal::TERM.lock().set_color(common_color::WHITE);
    print!(" v{}.{}.{}.{}\n\n",
           VERSION_MAJOR,
           VERSION_MID,
           VERSION_MINOR,
           VERSION_COMMIT);

    // Initialize frame allocation
    print!(" >> Initializing memory management\n");
    let alloc = memory::init_area_frame_alloc(mb_info_address);

    // Initialize PIC
    print!(" >> Initializing PIC\n");
    int::pic::remap(0x20, 0x28);
    // Temporarily mask PICs
    io::pio::outb(0x21, 0xfd);
    io::pio::outb(0x2a, 0xff);

    // Initialize IDT
    print!(" >> Initializing IDT\n");
    let mut idt = int::Idt::new();

    let divzero = int::Entry::new(0x0, 0x08, 0x8e);
    idt.add_isr(0x0, divzero);
    idt.install();

    loop {}
}
