//!-----------------------------------------------------------------------------------------------
//!`src/main.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![no_std]

#![feature(lang_items)]

use core::fmt::Write;

mod lib;
mod display;
mod memory;

pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MINOR: u16 = 1;
pub const VERSION_PATCH: u16 = 3;

#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {
    let color: u8 = display::Color::make_color(display::Color::Green, display::Color::Black);
    let mut term = display::terminal::Terminal::new();
	term.set_color(color);

    unsafe {
		term.goto_line(0);
		write!(term, "Modulon v{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
		term.goto_line(1);
		let mb_info = lib::multiboot::BootInfo::new(mb_info_address);
		write!(term, "Boot info location: {}, boot info size: {}", mb_info_address, mb_info.size);
		term.goto_line(2);
		write!(term, "First tag: {}", mb_info.first.typ);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
	loop{}
}
