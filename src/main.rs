#![feature(lang_items)]
#![no_std]

use core::fmt::Write;

mod display;
mod memory;

#[no_mangle]
pub extern "C" fn kmain(mb_info: u32) {
    let color: u8 = display::Color::make_color(display::Color::Green, display::Color::Black);
    let mut term = display::terminal::Terminal::new();
	term.set_color(color);

    unsafe {
		term.goto_line(0);
        write!(term, "                _      _          ");
		term.goto_line(1);
        write!(term, "  _ __  ___  __| |_  _| |___ _ _  ");
		term.goto_line(2);
        write!(term, " | '  \\/ _ \\/ _` | || | / _ \\ ' \\ ");
		term.goto_line(3);
        write!(term, " |_|_|_\\___/\\__,_|\\_,_|_\\___/_||_|");
		term.goto_line(5);
		write!(term, "Starting system...");
		term.goto_line(6);
		write!(term, "Boot info location: {}", mb_info);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
	loop{}
}
