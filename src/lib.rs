#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(naked_functions)]
#![no_std]

/// Multiboot crate for retrieving boot information
extern crate multiboot2;

/// Spinlock crate
extern crate spin;

#[macro_use]
/// Macros
pub mod macros;
/// x86_64 specific code
///
/// This module contains code specific to the x86_64 architecture such as
/// memory management, interrupts, etc.
pub mod x64;
/// Standard support library
///
/// Contains memory manipulation functions such as memcpy, memmove, memset, and memcmp.
/// Rust depends on these functions to compile.
pub mod support;
/// Architecture independent I/O drivers
///
/// Includes Port I/O and display drivers.
pub mod io;
/// Current environment information
///
/// Contains information such as system time, current thread, etc.
pub mod env;
/// Disk drivers
///
/// Drivers for reading and writing from storage devices.
pub mod disk;
/// VFS and filesystem drivers
///
/// Virtual file system abstraction and filesystem drivers
pub mod fs;
/// Rust panic_fmt function
pub mod panic;

/// Version information
pub const VERSION: &'static str = "0.1.8";

/// Reexport x64 architecture components
pub use x64::*;

use io::display::*;

/// Kernel main
///
/// This is the main kernel entry point. It is called by `src/asm/x64/lm_start.asm`.
/// This function initializes the kernel.
#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {

    // Clear text-mode terminal
    terminal::TERM.lock().clear();

    // Display version information
    terminal::TERM.lock().set_color(GREEN);
    print!("Modulus ");
    terminal::TERM.lock().set_color(WHITE);
    print!("{}\n\n", VERSION);

    // Initialize frame allocation
    print!(" >> Initializing memory management\n");

    let alloc = memory::init_area_frame_alloc(mb_info_address);

    // Initialize PIC
    print!(" >> Initializing PIC\n");
    int::pic::remap(0x20, 0x28);
    // Temporarily mask PICs
    io::pio::outb(0x21, 0xfc);
    io::pio::outb(0x2a, 0xff);

    // Initialize IDT
    print!(" >> Initializing IDT\n");
    int::init();

    env::time::init();
    // Initialize PIT
    print!(" >> Initializing PIT\n");

    terminal::TERM.lock().set_color(GREEN);
    print!("\nStartup time: {}ms\n", env::time::ms());
    terminal::TERM.lock().set_color(WHITE);

    let disk = match disk::ata::Ata::new(0x1f0, false) {
        Some(disk) => disk,
        None => disk::ata::Ata::new(0x1f0, true).expect("No disk"),
    };

    let fs = fs::iso9660::Iso9660::new(disk, 0x40);

    if fs.is_some() {
        print!("ISO9660 filesystem");
    } else {
        print!("Not an ISO9660 filesystem");
    }

    loop {}
}
