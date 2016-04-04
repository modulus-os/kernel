use core::fmt::Write;
use io::display::*;

// Use Rust core library
extern crate core;

pub fn panic(file: &str, line: u32) {
	terminal::TERM.lock().set_color(common_color::RED);
	print!("\n\n\n\n");
	error_symbol();
	print!("    !PANIC!    \n\n");
	terminal::TERM.lock().set_color(common_color::WHITE);
	print!("At {}:{}", file, line);
}

pub fn exception(err: &str) {
	let mut term = terminal::Terminal::new();

	term.set_color(common_color::RED);
	print!("\n\n\n\n");
	error_symbol();
	print!("  !EXCEPTION!  \n\n");
	term.set_color(common_color::WHITE);

	print!("{}", err);
}

// Bare error icon
fn error_symbol() {
	print!("       x       \n");
	print!("      x x      \n");
	print!("     x | x     \n");
	print!("    x  |  x    \n");
	print!("   x       x   \n");
	print!("  x    *    x  \n");
	print!(" x===========x \n");
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt(file: &str, line: u32) -> ! {
	panic(file, line);
	loop{}
}
