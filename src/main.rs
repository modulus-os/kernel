//!-----------------------------------------------------------------------------------------------
//!`src/main.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![no_std]
#![feature(lang_items)]
#![feature(asm)]

use core::fmt::Write;

mod support;
mod multiboot;
mod display;
mod memory;
mod cpuio;

use cpuio::Port;

pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MINOR: u16 = 1;
pub const VERSION_PATCH: u16 = 4;

#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {
    let color: u8 = display::Color::make_color(display::Color::Green, display::Color::Black);
    let mut term = display::terminal::Terminal::new();
	term.set_color(color);

	write!(term, "Modulon v{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
	term.newline();

	//panic!();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
	let color: u8 = display::Color::make_color(display::Color::Red, display::Color::Black);
	let mut term = display::terminal::Terminal::new();
	term.set_color(color);

	write!(term, "System panic!");
	loop{}
}
