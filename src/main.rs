//!-----------------------------------------------------------------------------------------------
//!`src/main.rs`
//!
//!Main rust file, declares all other modules to be included and implements kmain().
//!-----------------------------------------------------------------------------------------------

#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]

use core::fmt::Write;

pub mod support;
pub mod multiboot;
pub mod memory;
pub mod io;

use io::cpuio::Port;
use io::display::*;

pub const VERSION_MAJOR: u16 = 0;
pub const VERSION_MINOR: u16 = 1;
pub const VERSION_PATCH: u16 = 4;

const WHITE: u8 = Color::new(Color::White, Color::Black);
const GREEN: u8 = Color::new(Color::Green, Color::Black);
const RED: u8 = Color::new(Color::Red, Color::Black);

pub const CODE_SEG: u64 = (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53);

#[no_mangle]
pub extern "C" fn kmain(mb_info_address: usize) {
	let mut term = terminal::Terminal::new();
	term.set_color(WHITE);

	write!(term, "Modulon v{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
	term.newline();

	write!(term, "Initialize IDT:");

	io::interrupt::init_idt();

	end(true, &mut term);
}

fn end(success: bool, term: &mut terminal::Terminal) {
	if success {
		term.set_color(GREEN);
		write!(term, " OK");
	} else {
		term.set_color(RED);
		write!(term, " FAILED");
	}
	term.set_color(WHITE);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    let mut term = terminal::Terminal::new();
	term.set_color(RED);

	write!(term, "System panic!");
	loop{}
}
