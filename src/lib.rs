#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![no_std]

// Multiboot crate for retrieving boot information
extern crate multiboot2;

// Spinlock crate
extern crate spin;

#[macro_use]
// Bitflags crate
extern crate bitflags;

#[macro_use]
/// Macros
pub mod macros;

/// Architecture specific code
///
/// This module contains code that is not architecture independent, which would otherwise pollute
/// the main kernel code.
pub mod arch;

/// Common functions
///
/// Contains common kernel functions such as power management and panic handling.
pub mod common;

/// Standard support library
///
/// Contains memory manipulation functions such as memcpy, memmove, memset, and memcmp.
/// Rust depends on these functions to compile.
pub mod support;

// Version information
pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MID: u16 = 1;
pub const VERSION_MINOR: u16 = 6;
pub const VERSION_COMMIT: u16 = 5;

// Reexport x86_64 architecture components
pub use arch::x86_64::*;

use io::display::*;
use memory::alloc::FrameAlloc;

/// Kernel main
///
/// This is the main kernel entry point. It is called by `src/asm/x86_64/lm_start.asm`.
/// This function initializes the kernel.
#[no_mangle]
pub extern fn kmain(mb_info_address: usize) {
	// Create terminal for logging
	terminal::TERM.lock().clear();

	// Display version information
	terminal::TERM.lock().set_color(common_color::GREEN);
	print!("Modulon");
	terminal::TERM.lock().set_color(common_color::WHITE);
	print!(" v{}.{}.{}.{} Buttered Potato\n\n", VERSION_MAJOR, VERSION_MID,
		VERSION_MINOR, VERSION_COMMIT);

	// Initialize frame allocation
	print!(" >> Initializing memory management\n");
	let mut alloc = memory::init_area_frame_alloc(mb_info_address);
    
    // Test frame allocation
    alloc.alloc();

	// Initialization complete
}
