//!-----------------------------------------------------------------------------------------------
//!`src/lib.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]

#![no_std]

//Multiboot crate for retrieving boot information
extern crate multiboot2;

//Architecture specific code
pub mod arch;

//Standard support library
pub mod support;

//Miscellaneous utilities
pub mod utils;

//Error/panic handling
pub mod error;

//Commonly used colors
pub mod common_color {
	use io::display::Color;

	pub const WHITE: u8 = Color::new(Color::White, Color::Black);
	pub const GREEN: u8 = Color::new(Color::Green, Color::Black);
	pub const RED: u8 = Color::new(Color::Red, Color::Black);
	pub const LCYAN: u8 = Color::new(Color::LightCyan, Color::Black);
}

//Display imports
use core::fmt::Write;
use arch::x86_64::io::display::*;

//Version information
pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MID: u16 = 1;
pub const VERSION_MINOR: u16 = 5;
pub const VERSION_COMMIT: u16 = 3;

//Use x86_64 architecture components
pub use arch::x86_64::*;

//Initializes kernel
#[no_mangle]
pub extern fn kmain(mb_info_address: usize) {
	//Create terminal for logging
	let mut term = terminal::Terminal::new();

	//Display version
	term.set_color(common_color::GREEN);
	write!(term, " Modulon");
	term.set_color(common_color::WHITE);
	write!(term, " v{}.{}.{}.{}\n\n", VERSION_MAJOR, VERSION_MID, VERSION_MINOR, VERSION_COMMIT);

	utils::init_log::init_log(&mut term, "Init memory management", true);

	memory::init_frame_alloc(&mut term, mb_info_address);

	//Finished
	utils::init_log::init_log(&mut term, "Init complete", true);
}
