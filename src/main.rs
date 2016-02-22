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

#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {
	let VERSION = "0.1.2";

    let color: u8 = display::Color::make_color(display::Color::Green, display::Color::Black);
    let mut term = display::terminal::Terminal::new();
	term.set_color(color);

    unsafe {
		term.goto_line(0);
		write!(term, "Modulon v{}", VERSION);
		term.goto_line(1);
		let mb_info = multiboot::load(mb_info_address);

		write!(term, "Boot info location: {}, boot info size: {}", mb_info_address, mb_info.size);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
	loop{}
}
