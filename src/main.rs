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
pub mod multiboot;
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
pub const VERSION_COMMIT: u16 = 1;

#[no_mangle]
pub extern fn kmain(mb_info_address: usize) {
	//Create terminal for logging
	let mut term = terminal::Terminal::new();
	term.set_color(common_color::WHITE);

	//Display version
	write!(term, "Modulon v{}.{}.{}.{}\n\n", VERSION_MAJOR, VERSION_MID, VERSION_MINOR, VERSION_COMMIT);

	//Initialize the IDT
	log_init(&mut term, "Interrupts");

	io::interrupts::init_idt(&mut term);

	end(true, &mut term);

	//Finished
	write!(term, "\nINIT COMPLETE");
}

fn end(success: bool, term: &mut terminal::Terminal) {
	if success {
		term.set_color(common_color::GREEN);
		write!(term, " OK\n");
	} else {
		term.set_color(common_color::RED);
		write!(term, " FAILED\n");
	}
	term.set_color(common_color::WHITE);
}

fn log_init(term: &mut terminal::Terminal, name: &str) {
	term.set_color(common_color::LCYAN);
	write!(term, "INIT ");
	term.set_color(common_color::WHITE);
	write!(term, "{}", name);
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
