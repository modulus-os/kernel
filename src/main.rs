//!-----------------------------------------------------------------------------------------------
//!`src/main.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![no_std]

#![feature(lang_items)]

use core::fmt::Write;

mod support;
mod multiboot;
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

	write!(term, "Modulon v{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
	term.newline();

	let mb_info: &multiboot::BootInfo;
	unsafe {mb_info = multiboot::BootInfo::new(mb_info_address);}

	write!(term, "Memmap tag found: type = {}", mb_info.get_tag(6).unwrap().typ);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
	loop{}
}
