//!-----------------------------------------------------------------------------------------------
//!`src/error.rs`
//!
//!Handles kernel errors/panics
//!-----------------------------------------------------------------------------------------------

use core::fmt;
use core::fmt::Write;
use io::display::*;

use common_color;

//Use Rust core library
extern crate core;

pub fn panic(file: &str, line: u32) {
	let mut term = terminal::Terminal::new();

	term.set_color(common_color::RED);
	write!(term, "\n\n\n\n");
	error_symbol(&mut term);
	write!(term, "    !PANIC!    \n\n");

	term.set_color(common_color::WHITE);

	write!(term, "At {}:{}", file, line);
}

pub fn exception(err: &str) {
	let mut term = terminal::Terminal::new();

	term.set_color(common_color::RED);
	write!(term, "\n\n\n\n");
	error_symbol(&mut term);
	write!(term, "  !EXCEPTION!  \n\n");

	term.set_color(common_color::WHITE);

	write!(term, "{}", err);
}

//Bare error icon
fn error_symbol(term: &mut terminal::Terminal) {
	write!(term, "       x       \n");
	write!(term, "      x x      \n");
	write!(term, "     x | x     \n");
	write!(term, "    x  |  x    \n");
	write!(term, "   x       x   \n");
	write!(term, "  x    *    x  \n");
	write!(term, " x===========x \n");
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments,
	file: &str, line: u32) -> ! {
	panic(file, line);
	loop{}
}
