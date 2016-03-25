//!-----------------------------------------------------------------------------------------------
//!`src/main.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]

pub mod support;
pub mod utils;
pub mod memory;
pub mod io;
pub mod error;
pub mod common_color {
	use io::display::Color;

	pub const WHITE: u8 = Color::new(Color::White, Color::Black);
	pub const GREEN: u8 = Color::new(Color::Green, Color::Black);
	pub const RED: u8 = Color::new(Color::Red, Color::Black);
	pub const LCYAN: u8 = Color::new(Color::LightCyan, Color::Black);
}

use core::fmt::Write;
use io::display::*;

pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MID: u16 = 1;
pub const VERSION_MINOR: u16 = 5;
pub const VERSION_COMMIT: u16 = 2;

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

	//Finished
	utils::init_log::init_log(&mut term, "Init complete", true);
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments,
	file: &str, line: u32) -> ! {
	error::panic(file, line);
	loop{}
}
